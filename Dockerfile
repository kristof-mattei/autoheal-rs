FROM rust:1.67.1@sha256:9a61110d6f1a7bd6915afa8794134c778f545c8ea97ac0d10f01f1380da2c1b2 as builder

ENV TARGET=x86_64-unknown-linux-musl
RUN rustup target add ${TARGET}

RUN rm -f /etc/apt/apt.conf.d/docker-clean; echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

# borrowed (Ba Dum Tss!) from
# https://github.com/pablodeymo/rust-musl-builder/blob/7a7ea3e909b1ef00c177d9eeac32d8c9d7d6a08c/Dockerfile#L48-L49
RUN --mount=type=cache,target=/var/cache/apt --mount=type=cache,target=/var/lib/apt \
    apt-get update && \
    apt-get --no-install-recommends install -y \
    build-essential \
    musl-dev \
    musl-tools

# The following block
# creates an empty app, and we copy in Cargo.toml and Cargo.lock as they represent our dependencies
# This allows us to copy in the source in a different layer which in turn allows us to leverage Docker's layer caching
# That means that if our dependencies don't change rebuilding is much faster
WORKDIR /build
RUN cargo new autoheal-rs
WORKDIR /build/autoheal-rs
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,id=cargo-only,target=/build/autoheal-rs/target \
    cargo build --release --target ${TARGET}

# now we copy in the source which is more prone to changes and build it
COPY src ./src
# --release not needed, it is implied with install
RUN --mount=type=cache,id=full-build,target=/build/autoheal-rs/target \
    cargo install --path . --target ${TARGET} --root /output

FROM alpine:3.17.2@sha256:69665d02cb32192e52e07644d76bc6f25abeb5410edc1c7a81a10ba3f0efb90a

# We're explicitely wanting to be root, because most consumers will just
# run the container expecting it to work. Since Docker runs as root, we match
USER root

WORKDIR /app
COPY --from=builder /output/bin/autoheal-rs /app

ENV RUST_BACKTRACE=1
ENTRYPOINT ["/app/autoheal-rs"]

FROM rust:1.72.0@sha256:b166ff9bb67a77434603c3c1053ce7d3a57e44c5a650d47a6eb9017c56ae1825 as builder

ARG TARGET=x86_64-unknown-linux-musl
ARG APPLICATION_NAME

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
RUN cargo new ${APPLICATION_NAME}
WORKDIR /build/${APPLICATION_NAME}
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,id=cargo-dependencies,target=/build/${APPLICATION_NAME}/target \
    cargo build --release --target ${TARGET}

# now we copy in the source which is more prone to changes and build it
COPY src ./src
# --release not needed, it is implied with install
RUN --mount=type=cache,id=full-build,target=/build/${APPLICATION_NAME}/target \
    cargo install --path . --target ${TARGET} --root /output

FROM alpine:3.18.3@sha256:7144f7bab3d4c2648d7e59409f15ec52a18006a128c733fcff20d3a4a54ba44a

ARG APPLICATION_NAME

# We're explicitely wanting to be root, because most consumers will just
# run the container expecting it to work. Since Docker runs as root, we match
USER root

WORKDIR /app
COPY --from=builder /output/bin/${APPLICATION_NAME} /app/entrypoint

ENV RUST_BACKTRACE=full
ENTRYPOINT ["/app/entrypoint"]

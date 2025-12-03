FROM alpine:latest@sha256:51183f2cfa6320055da30872f211093f9ff1d3cf06f39a0bdb212314c5dc7375

RUN apk --update add bash docker

RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

WORKDIR /app
COPY . .

ENTRYPOINT ["/app/entrypoint.sh"]

FROM alpine:latest@sha256:a59bbcb5abd79dd098c5f2de7f603aa7a90edf2d04f7c4a6fecc4eac08836290

RUN apk --update add bash docker

RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

WORKDIR /app
COPY . .

ENTRYPOINT ["/app/entrypoint.sh"]

FROM alpine:latest@sha256:ac03b2a7eecaa3b1871d4c3971bf93dbd37ab9d69a4031b25eae3c8a9783f58a

RUN apk --update add bash docker


RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

WORKDIR /app
COPY . .

ENTRYPOINT ["/app/entrypoint.sh"]

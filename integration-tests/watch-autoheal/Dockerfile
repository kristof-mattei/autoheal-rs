FROM alpine:latest@sha256:02bb6f428431fbc2809c5d1b41eab5a68350194fb508869a33cb1af4444c9b11

RUN apk --update add bash docker


RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

WORKDIR /app
COPY . .

ENTRYPOINT ["/app/entrypoint.sh"]

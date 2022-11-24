FROM alpine:latest@sha256:8914eb54f968791faf6a8638949e480fef81e697984fba772b3976835194c6d4

RUN apk --update add bash docker


RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

WORKDIR /app
COPY . .

ENTRYPOINT ["/app/entrypoint.sh"]

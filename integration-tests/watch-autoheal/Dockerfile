FROM alpine:latest@sha256:51b67269f354137895d43f3b3d810bfacd3945438e94dc5ac55fdac340352f48

RUN apk --update add bash docker

RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

WORKDIR /app
COPY . .

ENTRYPOINT ["/app/entrypoint.sh"]

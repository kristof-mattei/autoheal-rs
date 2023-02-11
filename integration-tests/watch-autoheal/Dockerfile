FROM alpine:latest@sha256:c8deccde20bcc319a17c28ce674b80b9b7e945a8b460a11272b7f0a3e0d6a746

RUN apk --update add bash docker


RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

WORKDIR /app
COPY . .

ENTRYPOINT ["/app/entrypoint.sh"]

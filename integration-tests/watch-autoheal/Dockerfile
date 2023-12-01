FROM alpine:latest@sha256:69e0a8b6d2048be90290c992957a3cde2e87e955e5bb9c27d228c78c46c7840f

RUN apk --update add bash docker

RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

WORKDIR /app
COPY . .

ENTRYPOINT ["/app/entrypoint.sh"]

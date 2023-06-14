FROM alpine:latest@sha256:208379042bb6ae86ff71ac053e3c53108a16670111f907740dea604fb4628015

RUN apk --update add bash docker


RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

WORKDIR /app
COPY . .

ENTRYPOINT ["/app/entrypoint.sh"]

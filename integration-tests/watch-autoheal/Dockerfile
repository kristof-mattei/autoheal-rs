FROM alpine:latest@sha256:b2bf705cebbc5a03a0b58e49ec152261ea7da707f40f7d5db1c4f75ed72d43c1

RUN apk --update add bash docker

RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

WORKDIR /app
COPY . .

ENTRYPOINT ["/app/entrypoint.sh"]

#!/usr/bin/env bash
set -o errexit -o nounset -o xtrace -o pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"

COMPOSE_PROJECT_NAME=${1:-autoheal-rs-test}
export COMPOSE_PROJECT_NAME

AUTOHEAL_CONTAINER_LABEL=autoheal-rs-test
export AUTOHEAL_CONTAINER_LABEL

COMPOSE_FILE="docker-compose.yml:docker-compose.watch.yml:"

if [[ -n ${IMAGE_ID+x} ]]; then
    # CI sets IMAGE_ID to the image it built
    COMPOSE_FILE+="docker-compose.image.yml"
else
    # the Dockerfile needs version-bump.patch
    touch ../version-bump.patch

    COMPOSE_FILE+="docker-compose.build.yml"
fi

export COMPOSE_FILE

docker compose config

function cleanup() {
    exit_status=$?
    echo "exit was $exit_status"
    # prevent autoheal-rs from restarting test containers while they stop
    docker compose stop autoheal-rs
    if (( exit_status != 0 )); then
        docker compose logs autoheal-rs
    fi
    docker compose down --timeout 1 || true
    exit "$exit_status"
}

trap cleanup EXIT
docker compose build
docker compose up --no-start --quiet-pull --force-recreate

docker compose start unhealthy-labeled healthy-labeled unhealthy-unlabeled unhealthy-excluded autoheal-rs

docker compose up --abort-on-container-exit --exit-code-from watch-autoheal-rs watch-autoheal-rs

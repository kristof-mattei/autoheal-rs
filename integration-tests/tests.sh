#!/usr/bin/env bash
set -euxo pipefail

COMPOSE_PROJECT_NAME=${1:-autoheal-test}
export COMPOSE_PROJECT_NAME

COMPOSE_FILE="docker-compose.yml:docker-compose.autoheal.yml:"

if ! [[ -z ${IMAGE_ID+x} ]]; then
    # image id is from built container when ran via GitHub actions. See build.yml
    COMPOSE_FILE+="docker-compose.image.yml"
else
    # build ourselves
    COMPOSE_FILE+="docker-compose.build.yml"
fi

export COMPOSE_FILE

docker compose config

function cleanup() {
    exit_status=$?
    echo "exit was $exit_status"
    # stop autoheal first, to stop it restarting the test containers while we try to stop them
    docker compose stop autoheal
    docker compose down || true
    exit "$exit_status"
}

trap cleanup EXIT
docker compose build
docker compose up --no-start --quiet-pull --force-recreate

docker compose start should-keep-restarting
docker compose start shouldnt-restart-healthy
docker compose start shouldnt-restart-no-label
docker compose start ignore
docker compose start autoheal

docker compose up --abort-on-container-exit --exit-code-from watch-autoheal watch-autoheal

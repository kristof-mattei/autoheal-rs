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
    # stop autoheal-rs first, to stop it restarting the test containers while we try to stop them
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

docker compose start should-keep-restarting shouldnt-restart-healthy shouldnt-restart-no-label ignore autoheal-rs

docker compose up --abort-on-container-exit --exit-code-from watch-autoheal-rs watch-autoheal-rs

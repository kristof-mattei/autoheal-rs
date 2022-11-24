#!/usr/bin/env bash
set -euxo pipefail

COMPOSE_PROJECT_NAME=${1:-autoheal-test}
export COMPOSE_PROJECT_NAME

files="-f docker-compose.yml -f docker-compose.autoheal.yml"

function cleanup() {
    exit_status=$?
    echo "exit was $exit_status"
    # stop autoheal first, to stop it restarting the test containers while we try to stop them
    docker-compose $files stop autoheal
    docker-compose $files down || true
    exit "$exit_status"
}

trap cleanup EXIT
docker-compose $files build
docker-compose $files up --no-start --quiet-pull --force-recreate
# docker compose $files logs --follow &

docker-compose $files start should-keep-restarting
docker-compose $files start shouldnt-restart-healthy
docker-compose $files start shouldnt-restart-no-label
docker-compose $files start autoheal # & # run for the logs

docker-compose $files up --exit-code-from watch-autoheal watch-autoheal

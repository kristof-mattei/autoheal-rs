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

# autoheal drops its capabilities right after starting, give it a moment
sleep 1

autoheal_status="/proc/$(docker inspect --format '{{.State.Pid}}' "$(docker compose ps --quiet autoheal)")/status"

# forgive me
grep --extended-regexp '^(Uid|Cap(Inh|Prm|Eff|Bnd|Amb)|NoNewPrivs):' "${autoheal_status}"

test "$(grep --extended-regexp --count '^Cap(Inh|Prm|Eff|Bnd|Amb):\s+0000000000000000$' "${autoheal_status}")" -eq 5
grep --quiet --extended-regexp '^NoNewPrivs:\s+1$' "${autoheal_status}"

docker compose up --abort-on-container-exit --exit-code-from watch-autoheal watch-autoheal

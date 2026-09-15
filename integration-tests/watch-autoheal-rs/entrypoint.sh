#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

: "${COMPOSE_PROJECT_NAME:?}"

TIMEOUT_SECONDS=15
# autoheal-rs polls every second, three quiet polls prove it left a container alone
SETTLE_SECONDS=3

# Docker sets `.State.StartedAt` on every start, this holds the last one seen per container
declare -A started

# container_name <service>
container_name() {
    echo "${COMPOSE_PROJECT_NAME}-${1}-1"
}

# started_at <container>
started_at() {
    docker inspect --format '{{ .State.StartedAt }}' "${1}"
}

# health <container>
health() {
    docker inspect --format '{{ .State.Health.Status }}' "${1}"
}

# make_unhealthy <container>
make_unhealthy() {
    docker exec "${1}" touch /flags/unhealthy
    echo "made ${1} unhealthy"
}

# expect_health <status> <container>: polls until the health status matches
expect_health() {
    local expected=$1 container=$2
    local deadline=$((SECONDS + TIMEOUT_SECONDS))
    local actual

    while true; do
        actual=$(health "${container}")

        if [[ ${actual} == "${expected}" ]]; then
            echo "OK: ${container} is ${actual}"
            return 0
        fi

        if ((SECONDS >= deadline)); then
            echo "ERR: ${container}: expected ${expected}, got ${actual}" >&2
            return 1
        fi

        sleep 1
    done
}

# expect_restart <container>: polls until the container started anew, then records that start
expect_restart() {
    local container=$1
    local deadline=$((SECONDS + TIMEOUT_SECONDS))
    local actual

    while true; do
        actual=$(started_at "${container}")

        if [[ ${actual} != "${started[${container}]}" ]]; then
            started[${container}]=${actual}
            echo "OK: ${container} restarted"
            return 0
        fi

        if ((SECONDS >= deadline)); then
            echo "ERR: ${container}: no restart within ${TIMEOUT_SECONDS}s" >&2
            return 1
        fi

        sleep 1
    done
}

# expect_no_restart <containers...>: the recorded starts hold for SETTLE_SECONDS
expect_no_restart() {
    local end=$((SECONDS + SETTLE_SECONDS))
    local container

    while ((SECONDS < end)); do
        sleep 1

        for container in "$@"; do
            if [[ $(started_at "${container}") != "${started[${container}]}" ]]; then
                echo "ERR: ${container} restarted" >&2
                return 1
            fi
        done
    done

    echo "OK: no restart of $*"
}

main() {
    local unhealthy_labeled healthy_labeled unhealthy_unlabeled unhealthy_excluded
    unhealthy_labeled=$(container_name unhealthy-labeled)
    healthy_labeled=$(container_name healthy-labeled)
    unhealthy_unlabeled=$(container_name unhealthy-unlabeled)
    unhealthy_excluded=$(container_name unhealthy-excluded)

    local container
    for container in "${unhealthy_labeled}" "${healthy_labeled}" "${unhealthy_unlabeled}" "${unhealthy_excluded}"; do
        started[${container}]=$(started_at "${container}")
    done

    echo "=== unhealthy without the label or excluded by name ==="
    make_unhealthy "${unhealthy_unlabeled}"
    make_unhealthy "${unhealthy_excluded}"
    expect_health unhealthy "${unhealthy_unlabeled}"
    expect_health unhealthy "${unhealthy_excluded}"

    echo "=== unhealthy with the label: restarted, healthy again, then left alone ==="
    make_unhealthy "${unhealthy_labeled}"
    expect_restart "${unhealthy_labeled}"
    expect_health healthy "${unhealthy_labeled}"
    expect_no_restart "${unhealthy_labeled}"

    echo "=== unhealthy again: restarted again ==="
    make_unhealthy "${unhealthy_labeled}"
    expect_restart "${unhealthy_labeled}"

    echo "=== the others were never restarted ==="
    expect_no_restart "${healthy_labeled}" "${unhealthy_unlabeled}" "${unhealthy_excluded}"

    echo "OK: all checks passed"
}

main

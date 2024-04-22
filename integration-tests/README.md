# Docker Autoheal Tests

Docker autoheal monitor written in Rust, inspired by [docker-autoheal](https://github.com/willfarrell/docker-autoheal).

Code currently uses hand-written Docker API calls, [hyper-unix-socket](https://github.com/kristof-mattei/hyper-unix-socket) to talk to Docker over a Unix socket and Tokio to be the glue.

Docker Compose is used to build and deploy test environment.

test.sh waits on watch-autoheal exit code.

Currently setup to a very basic exit 1 on invalid restart and exit 0 on valid restart.

## Run tests

```
cd integration-tests
./tests.sh
```

## Run tests in CI

```
cd integration-tests
export "AUTOHEAL_CONTAINER_LABEL=autoheal-123456"
./tests.sh "MY_UNIQUE_BUILD_NUMBER_123456"
```

This enables the tests to only restart containers within the test spec by using
unique docker-compose project names and autoheal labels (as long as you replace
123456 by a unique number)

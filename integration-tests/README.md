# autoheal-rs integration tests

Docker Compose runs autoheal-rs next to alpine containers with passing and failing health checks. A watcher tails the Docker events until the first restart.

```
./integration-tests/tests.sh
```

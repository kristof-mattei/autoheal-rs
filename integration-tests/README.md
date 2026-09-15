# autoheal-rs integration tests

Docker Compose runs autoheal-rs next to alpine containers with passing and failing health checks. A watcher makes containers unhealthy and checks which ones autoheal-rs restarts.

```
./integration-tests/tests.sh
```

# Autoheal-rs

Heals your Docker containers

## Running

Only the container is supported.

```yaml
services:
  autoheal:
    image: ghcr.io/kristof-mattei/autoheal-rs:latest
    restart: unless-stopped
    cap_drop:
      - ALL
    security_opt:
      - no-new-privileges:true
    volumes:
      - type: bind
        source: /var/run/docker.sock
        target: /var/run/docker.sock
    environment:
      # `autoheal` matches containers labeled `autoheal=true`, `all` matches every container
      AUTOHEAL_CONTAINER_LABEL: autoheal
```

The socket gives the container root on the host.

## License

MIT, see [LICENSE](./LICENSE)

`SPDX-License-Identifier: MIT`

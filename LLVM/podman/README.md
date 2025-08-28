# Demo Podman environment

## Demo without build artifacts

Estimated size: 2.5GB

To build: `podman build ./ -f ./Containerfile -t llcap-env`

To run: `podman run -it llcap-env`

### Push & update notes

* better to rebuild from scratch (or use a constant build argument)
* build and push `vasutro/llcap-demo-env:Vx:Vy:Vz`
* update [here](../README.md#containers) and [here](../sandbox/02-ipc/example-arg-replacement/README.md#prerequisites)

## Development version (not adapted to this repository yet!)

Estimated size: 18.3GB

To build: `podman build ./ -f ./Containerfile-dev -t llcap-devenv`

To run: `podman run -it llcap-devenv`

You can use the `LLCAP_PROJECT_COMMIT` flag for `podman build`:

```
--build-arg LLCAP_PROJECT_COMMIT="commit hash here"
```

to control the rebuild of the container by forcing the checkout of a particular commit hash of this repository. Exercise caution though (or use `--no-cache`).

Also use `/bin/bash` in shebangs. It is available, others might not be.

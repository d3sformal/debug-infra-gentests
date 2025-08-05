# Demo Podman environment

## Demo without build artifacts

Estimated size: 2.2GB

To build: `podman build ./ -f ./Containerfile -t llcap-env`

To run: `podman run -it llcap-env`

### Running end-to-end tests

To run [e2e tests](../sandbox/02-ipc/e2e-tests/), you need to (**inside** the container) 
create the `[llcap-server](../sandbox/02-ipc/llcap-server/)/target/debug` directory and
`cp llcap-server/bin/llcap-server llcap-server/target/debug`.

### Push & update notes

* better to rebuild from scratch (or use a constant build argument)
* build and push `vasutro/llcap-demo-env:Vx:Vy:Vz`
* update [here](../README.md#containers) and [here](../sandbox/02-ipc/example-arg-replacement/README.md#prerequisites)

## Development version

Estimated size: 18.3GB

To build: `podman build ./ -f ./Containerfile-dev -t llcap-devenv`

To run: `podman run -it llcap-devenv`

You can use the following flags 

```
--build-arg BUILDCLONE="`date`"
--build-arg NONLLVMUPDATE="`date`"
```

to control the rebuild of the container in a particular stage.

`BUILDCLONE` rebuilds the entire container, including LLVM.
`NONLLVMUPDATE` pull from this repository right after (in-container) 
LLVM installation. Use at your own risk (or perform final verification via 
`BUILDCLONE`)

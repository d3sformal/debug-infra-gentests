# Demo Podman environment

## Demo without build artifacts

Estimated size: 750MB

To build: `podman build ./ -f ./Containerfile -t llcap-env`

To run: `podman run -it llcap-env`


## Development version

Estimated size: 18.5GB

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
# PP

A pixi extension that allows a user to push and pull (pp) pixi.toml/pixi.lock file pairs to/from remote OCI compatible registries.

## Dev setup

This project is setup using pixi! In order to run this locally you will need to:

1. setup a local OCI compatible registry
2. build/run the project with cargo

# 1: Start a local registry

To start a local registry server run the command
```
$ pixi run start-docker
```

# 2: Run the project

You may run the project directly with pixi
```
$ pixi run start push . localhost:5000/myimage
```

Or, if you want to run the binary you can add the target build directory to path, and build using pixi
```
$ export PATH=$PATH:$PWD/target/debug
$ pixi run build
$ pp push . localhost:5000/myimage
```

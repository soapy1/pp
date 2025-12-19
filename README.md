# PP (Push/Pull)

A pixi extension that allows a user to push and pull (pp) pixi.toml/pixi.lock file pairs to/from remote OCI compatible registries.

## Dev setup

This project is setup using pixi! In order to run this locally you will need to:

1. setup a local OCI compatible registry
2. build/run the project with cargo

### 1: Start a local registry

To start a local registry server run the command
```
$ pixi run start-docker
```

### 2: Run the project

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

## Try it out

Once you have build to project, try it out as a standalone application. In these examples, we'll push to a server running on port 5000. Remember to have the local registry started.

### Push a pixi project

In order to push a project to the remote repo, you can use the `push` command. In this example we will push up the artifact pp:v1 to the repository living at localhost:5000.

```
$ pp push ./path/to/my/pixi/project localhost:5000/pp:v1
```

You can verify the manifest on the remote using the `oras` cli.
```
$ pixi run -- oras manifest fetch localhost:5000/pp:v1 

{
  "annotations": {
    "org.opencontainers.image.description": "localhost:5000/pp:v1"
  },
  "config": {
    "digest": "sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a",
    "mediaType": "application/vnd.pixi.config.v1+toml",
    "size": 2
  },
  "layers": [
    {
      "annotations": {
        "org.opencontainers.image.title": "pixi.toml"
      },
      "digest": "sha256:5e45099776d1fb4ecf9f1e551c33a886e6c20a55ae18df5c285dd7608bd8c6f3",
      "mediaType": "application/vnd.pixi.toml.v1+toml",
      "size": 452
    },
    {
      "annotations": {
        "org.opencontainers.image.title": "pixi.lock"
      },
      "digest": "sha256:6845055a0b5743cf35109ed1296637cf9922d12cd66e4d250b60d023dd0c3471",
      "mediaType": "application/vnd.pixi.lock.v1+yaml",
      "size": 19273
    }
  ],
  "schemaVersion": 2
}
```

### Pull a pixi project

In order to pull a project to the remote repo, you can use the `pull` command. In this example we will pull the artifact we pushed up in the previous step.

```
$ pp pull ./dest localhost:5000/pp:v1
```

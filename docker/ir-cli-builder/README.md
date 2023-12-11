## The Smart IR Builder Image

The `smartir/smart-ir-builder` image embeded a development enviroment for developers to code, build and test the project.

### Develop

To develop the `Smart-Intermediate-Representation` project with docker:

```sh
# 1. clone the project locally
git clone git@github.com:AntChainOpenLabs/Smart-Intermediate-Representation.git

# 2. start the container, note to replace the ${path-to-Smart-Intermediate-Representation-project} to your local path
docker run --platform linux/x86_64 -v ${path-to-Smart-Intermediate-Representation-project}:/home/Smart-Intermediate-Representation-project -ti smartir/smart-ir-builder:v0.1.0 bash

# 3. code, build, test in your container:
cd /home/Smart-Intermediate-Representation-project
make install-rustc-wasm
make run-debug
```

### Build the image locally

Under the top level of the `Smart-Intermediate-Representation` repo and run the script below, it will build a `smartir/smart-ir-builder:latest` image locally:

```sh
./docker/ir-cli-builder/docker_build.sh
```

### Notes About the Image

- For Mac Arm64 user, please explictly specify the option `--platform linux/x86_64` to `docker build` and `docker run`, to avoid the "missing /lib64/ld-linux-x86-64.so.2" error. Ref: https://stackoverflow.com/a/69075554/12110648

- Since the `ir_cli` crate depends on the `smart_ir_macro` and `smart_ir` crates and are refered with relative paths(as shown bellow), this two dependencies will be removed when building the `smartir/smart-ir-builder` image. The `docker_build.sh` script does the removing automatically.

    ```toml
    smart_ir_macro = { path = "../smart_ir_macro", version = "0.3.0" }
    smart_ir = { path = "../smart_ir" }
    ```
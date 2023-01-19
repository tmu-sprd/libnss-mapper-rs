# libnss-mapper-rs

## Build

### With Rust installed

#### Preparation

You need the nightly toolchain and rust-src component installed.  
Run:

```shell
rustup toolchain install nightly --allow-downgrade --profile minimal --component rust-src
```

To create Debian packages you need crate cargo-deb.  
Run:

```shell
cargo install cargo-deb
```

#### Compile

To compile a size optimized binary, run:

```shell
cargo build_slim
```

To compile a size optimized binary without the syslog feature, run:

```shell
cargo build_slim --no-default-features
```

To compile a size optimized binary and create a Debian package, run:

```shell
cargo deb --cargo-build build_slim
```

To compile a size optimized binary without the syslog feature and create a Debian package, run:

```shell
cargo deb --cargo-build build_slim --variant=no-syslog
```

### Using Docker/Podman

If you don't have Rust installed or don't want to install the nightly toolchain, there is a script `./build.sh`, which creates an image from Debian Bullseye and with the needed Rust toolchain.

Invoke it as `./build.sh <command>`, where \<command\> is one of the above.
Example:

```shell
./build.sh cargo deb --cargo-build build_slim
```

To start a container with an interactive shell in the working directory, run:

```shell
./build.sh shell
```

To remove the built image, run:

```shell
./build.sh clean
```

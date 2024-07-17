



```
git clone git@github.com:embassy-rs/teleprobe.git
cd teleprobe/teleprobe
```

## Cross.toml
```
# Custom image
[target.aarch64-unknown-linux-gnu]
image = "elpiel/raspberrypi-os-cross:bookworm"
# image = "dtcooper/raspberrypi-os:bookworm"
pre-build = [
    "dpkg --add-architecture $CROSS_DEB_ARCH",
    "apt update",
    "apt upgrade -y",
    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh",
    # probe-rs with openssl
    "apt install -y pkg-config libusb-1.0-0-dev:$CROSS_DEB_ARCH libftdi1-dev:$CROSS_DEB_ARCH libudev-dev:$CROSS_DEB_ARCH libssl-dev:$CROSS_DEB_ARCH",
]

# Default image - libssl differes...
# [target.aarch64-unknown-linux-gnu]
# pre-build = [
#     "dpkg --add-architecture $CROSS_DEB_ARCH",
#     "apt update",
#     "apt upgrade -y",
#     # openssl
#     "apt install --assume-yes libssl-dev:$CROSS_DEB_ARCH libssl1.1:$CROSS_DEB_ARCH",
#     # probe-rs
#     "apt install -y pkg-config libusb-1.0-0-dev:$CROSS_DEB_ARCH libftdi1-dev:$CROSS_DEB_ARCH libudev-dev:$CROSS_DEB_ARCH libssl-dev:$CROSS_DEB_ARCH",
#     # libssl-dev libusb-1.0-0-dev libftdi1-dev libudev-dev
# ]
```

## Cross compilation for Raspberry Pi OS:

For `error: failed to run custom build command for \`libc v0.2.151\`` error and the missing llvm lib:
`failed to run rustc: rustc: error while loading shared libraries: libLLVM-17-rust-1.77.0-nightly.so: cannot open shared object file: No such file or directory`

You need conda:

Installation for Linux from repository: https://docs.conda.io/projects/conda/en/latest/user-guide/install/rpm-debian.html

Run:
```
conda install -c conda-forge gcc=12.1.0
conda install -c conda-forge libstdcxx-ng=12
```

And then build:

```
cross build --target aarch64-unknown-linux-gnu --release
```



```
scp target/aarch64-unknown-linux-gnu/debug/teleprobe pi@$PI_IP:/home/pi/teleprobe2

scp target/armv7-unknown-linux-gnueabihf/debug/teleprobe pi@$PI_IP:/home/pi/teleprobe3
```


`git clone https://github.com/dtcooper/raspberrypi-os-docker/`

```bash
docker build -t elpiel/raspberrypi-os-cross:bookworm --build-arg="BASE_CONTAINER=debian:bookworm" --platform linux/amd64 .
```

```bash
docker build -t elpiel/raspberrypi-os-cross:bookworm --build-arg="BASE_CONTAINER=dtcooper/raspberrypi-os:bookworm" --platform linux/amd64 .
```

Amd64:
`docker build -t elpiel/raspberrypi-os-cross:bookworm --platform linux/amd64 -f rust-rp-Dockerfile .`
Arm64:
`docker build -t  --platform linux/arm64 -f rust-rp-Dockerfile .`

elpiel/raspberrypi-os-cross:bookworm

### Building Rust / RP docker image

Use [rust-rp-Dockerfile](../rust-rp-Dockerfile):

Build and push for 2 platforms (`linux/amd64` & `linux/arm64` / `aarch64-*` targets):

```
docker buildx build -t elpiel/raspberrypi-os-cross:bookworm --platform linux/amd64,linux/arm64 -f rust-rp-Dockerfile --push .
```


Build...

```
cross build -r --target aarch64-unknown-linux-gnu
```
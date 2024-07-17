# ARG BASE_CONTAINER
# FROM $BASE_CONTAINER
# Force the usage of arm64 even if we are building for `linux/amd64`
FROM --platform=linux/arm64 dtcooper/raspberrypi-os:bookworm

ARG CROSS_DEB_ARCH=arm64

# COPY image/ /

# RUN dpkg --add-architecture armhf \
# RUN dpkg --add-architecture amd64
RUN dpkg --add-architecture $CROSS_DEB_ARCH
RUN echo "deb http://archive.raspberrypi.org/debian/ $(sh -c '. /etc/os-release; echo $VERSION_CODENAME') main" > /etc/apt/sources.list.d/raspi.list
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get upgrade -y \
    # && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    # this apt install command installs arm64 libraries
    && apt install -y curl git gcc pkg-config libssl-dev libssl1.1 libusb-1.0-0-dev libftdi1-dev libudev-dev \
    # apt install --assume-yes libssl-dev:$CROSS_DEB_ARCH libssl1.1:$CROSS_DEB_ARCH pkg-config libusb-1.0-0-dev:$CROSS_DEB_ARCH libftdi1-dev:$CROSS_DEB_ARCH libudev-dev:$CROSS_DEB_ARCH libssl-dev:$CROSS_DEB_ARCH \
    #     libraspberrypi0:amd64 \
    #     libraspberrypi-bin:amd64 \
    # && rm -rf /var/lib/apt/lists/* \
    # Install the Rust compiler
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# export cargo bin path.
# RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"


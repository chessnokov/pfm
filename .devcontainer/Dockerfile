# [Choice] Alpine version: 3.16, 3.15, 3.14, 3.13
ARG VARIANT=3.16
FROM docker.io/library/alpine:${VARIANT}

# [Option] Install zsh
ARG INSTALL_ZSH="true"

# Install needed packages and setup non-root user. Use a separate RUN statement to add your own dependencies.
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID
COPY library-scripts/*.sh library-scripts/*.env /tmp/library-scripts/
RUN apk update && ash /tmp/library-scripts/common-alpine.sh "${INSTALL_ZSH}" "${USERNAME}" "${USER_UID}" "${USER_GID}" \
    && rm -rf /tmp/library-scripts

# ** [Optional] Uncomment this section to install additional packages. **
RUN apk update \
    && apk add --no-cache openocd gcc-arm-none-eabi gdb-multiarch

ENV RUSTUP_HOME=/usr/local/rustup \
    PATH=/usr/local/cargo/bin:$PATH
ARG RUST_VERSION=1.65.0
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | CARGO_HOME=/usr/local/cargo sh -s -- -y --default-toolchain ${RUST_VERSION} --profile default -c rust-src -c clippy -c rustfmt -t armv7r-none-eabihf

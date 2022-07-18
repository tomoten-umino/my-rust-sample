FROM rust:1.59-buster

RUN apt-get update && \
    apt-get -y install git vim sudo && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rustup component add rls rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch

ARG USERNAME=tomoyoshi
ARG GROUPNAME=tomoyoshi
ARG UID=1000
ARG GID=1000
ARG PASSWORD=58daiAandC
RUN groupadd -g $GID $GROUPNAME && \
    useradd -m -s /bin/bash -u $UID -g $GID -G sudo $USERNAME && \
    echo $USERNAME:$PASSWORD | chpasswd && \
    echo "$USERNAME   ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

USER $USERNAME

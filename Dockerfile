FROM ubuntu:latest

RUN \
    apt-get update && apt-get -y upgrade && \
    apt-get install -y curl gcc && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh && \
    sh rustup.sh -y && . $HOME/.cargo/env && \
    cargo install spacework && \
    spacework --help

CMD ["bash"]

FROM rust:bullseye

COPY . .

RUN cargo install --path . && spacework --help

CMD ["bash"]

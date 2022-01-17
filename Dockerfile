FROM rust:1-bullseye AS builder
WORKDIR /uwu
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bullseye-slim AS runner
RUN apt update \
    && apt full-upgrade -y \
    && apt install ca-certificates -y \
    && apt autoremove --purge -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /uwu/target/release/uwu-bot /usr/bin/uwu-bot
RUN useradd uwu

USER uwu
ENTRYPOINT [ "/usr/bin/uwu-bot" ]

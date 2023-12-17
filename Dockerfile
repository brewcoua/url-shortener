FROM rust:slim-bullseye as builder

WORKDIR /prod
COPY Cargo.toml Cargo.lock ./
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim as runtime

RUN useradd -ms /bin/bash app
USER app
WORKDIR /home/app

COPY --from=builder /prod/target/release/url-shortener ./bin

CMD ["./bin"]

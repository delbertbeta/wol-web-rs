FROM rust:1.58.1-alpine3.14 as builder
RUN apk add musl-dev git

# create a new empty shell project
RUN USER=root cargo new --bin wol-web-rs
WORKDIR /wol-web-rs

COPY ./Cargo.lock ./Cargo.toml ./
COPY ./.cargo ./.cargo

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY templates ./templates

RUN rm ./target/release/deps/wol_web_rs*
RUN cargo build --release --verbose

FROM alpine as runtime
COPY --from=builder /wol-web-rs/target/release/wol-web-rs .
CMD ["./wol-web-rs"]

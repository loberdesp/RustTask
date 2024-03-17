FROM rust:latest

WORKDIR /RustTask

COPY . .

RUN cargo build --release

CMD cargo run
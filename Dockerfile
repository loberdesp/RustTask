FROM rust:latest as builder

WORKDIR /RustTask

COPY . .

RUN cargo build --release

FROM rust:latest

WORKDIR /RustTask

COPY --from=builder /RustTask .

CMD ["cargo", "test"]
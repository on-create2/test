FROM rust:1.85

WORKDIR /app

COPY Cargo.toml ./

COPY src ./src

RUN cargo build --release

CMD ["./target/release/hello_world"]
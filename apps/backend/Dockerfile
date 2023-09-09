FROM rust:latest

COPY . .

RUN cargo build --release

EXPOSE 8000

CMD ["./target/release/backend"]
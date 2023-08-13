FROM debian:bookworm-slim as builder

RUN apt-get update -y && apt-get install -y \
  curl build-essential pkg-config libssl-dev

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY . .
RUN ./setup.sh && cargo build --release

FROM debian:bookworm-slim as runner

WORKDIR /app
COPY --from=builder /app/target/release/maze-server /app/
COPY App.toml /app/

ENV RUST_LOG="debug"
CMD ["/app/maze-server"]

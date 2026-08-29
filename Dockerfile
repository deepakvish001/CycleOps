FROM rust:1.85-bookworm AS backend
WORKDIR /workspace
COPY Cargo.toml rust-toolchain.toml ./
COPY crates ./crates
RUN cargo build --release -p cycleops-api

FROM debian:bookworm-slim
RUN useradd --system --uid 10001 cycleops
COPY --from=backend /workspace/target/release/cycleops-api /usr/local/bin/cycleops-api
USER cycleops
EXPOSE 8080
ENTRYPOINT ["cycleops-api"]

FROM rust:1.73 AS build
WORKDIR /app
COPY Cargo.toml ./
COPY src/ ./src/
RUN apt-get update && apt-get install -y \
    musl-tools
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch AS final
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/rusty_adventures /rusty_adventures
ENTRYPOINT ["/rusty_adventures"]
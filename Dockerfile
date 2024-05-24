FROM rust:slim AS build
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:12-slim AS final
WORKDIR /app

COPY --from=build /app/target/release/azure-container-deployment ./

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
CMD ["./azure-container-deployment"]
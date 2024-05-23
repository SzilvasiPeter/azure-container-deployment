FROM rust:slim AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release
RUN cp target/release/azure-container-deployment /bin/webserver

FROM debian:12-slim AS final

WORKDIR /app

COPY --from=build /app/target/release/azure-container-deployment ./

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["./azure-container-deployment"]
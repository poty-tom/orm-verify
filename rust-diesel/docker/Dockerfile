FROM rust:1.57.0 as develop
WORKDIR /app
RUN cargo install cargo-watch
RUN apt install -y libpq-dev
RUN cargo install diesel_cli
COPY . .

FROM develop as build
RUN cargo build --release

FROM rust:1.57.0-slim-buster as prod
RUN apt-get update
RUN apt-get install libpq-dev -y
COPY --from=build-stage /app/target/release/api .
EXPOSE 8080
CMD ["/usr/local/bin/api"]
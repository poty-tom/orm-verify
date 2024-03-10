FROM rust:1.57.0

WORKDIR /app
RUN cargo install cargo-watch
RUN apt install -y libpq-dev
RUN cargo install diesel_cli
RUN cargo build

EXPOSE 8080
# WORKDIRベースでビルド時に生成されたバイナリを実行する
CMD ["/app/target/debug/rust-diesel"]

# FROM rust:1.57.0 as prod
# RUN apt-get update
# RUN apt-get install libpq-dev -y
# build stageの内容をprod stageの現在のWORKDIRにコピーする
# COPY --from=build-stage /app/target/release/api . 
# どうやら既定で/usr/local/bin/配下らしい。なのでコピーしたバイナリをコマンドとして渡して実行するだけ
# CMD ["/usr/local/bin/api"]

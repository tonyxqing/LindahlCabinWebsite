FROM rust:latest

RUN rm -rf /usr/src/app

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

EXPOSE 8000

CMD ./target/release/server

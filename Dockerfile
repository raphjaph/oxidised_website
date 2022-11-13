FROM rust:latest

RUN apt-get update -y

RUN cargo install zola

WORKDIR /website

COPY . .

RUN cargo build

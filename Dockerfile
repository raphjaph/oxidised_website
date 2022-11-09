FROM rust:slim

RUN apt-get update -y

WORKDIR /website
COPY . .

RUN cargo install zola

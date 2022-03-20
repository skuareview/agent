# 1. This tells docker to use the Rust official image
FROM debian

RUN apt-get update && apt install curl build-essential gcc make -y

RUN mkdir /app
WORKDIR /app

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"


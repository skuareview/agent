# 1. This tells docker to use the Rust official image
FROM debian

RUN apt-get update && apt install curl build-essential gcc make -y
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -y

# 3. Copy the files in your machine to the Docker image
RUN mkdir /app
COPY . /app
WORKDIR /app

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"


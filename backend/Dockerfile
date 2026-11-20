FROM rust:1 AS build
WORKDIR /app

#Install CURL
RUN apt-get update && apt-get install -y curl

#Download stockfish
RUN apt-get update && apt-get install -y curl tar \
    && mkdir -p /opt/stockfish \
    && curl -L https://github.com/official-stockfish/Stockfish/releases/download/sf_19/stockfish-linux-x86-64-universal.tar.gz \
       | tar -xz -C /opt/stockfish

RUN mv /opt/stockfish/stockfish/stockfish-linux-x86-64-universal /usr/local/bin/stockfish \
    && chmod +x /usr/local/bin/stockfish

COPY . .
RUN cargo build --release
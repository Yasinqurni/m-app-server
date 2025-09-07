FROM rust:latest

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml ./
COPY Cargo.lock ./
COPY .env ./

COPY . .

RUN cargo build --release

EXPOSE 3001

CMD ["./target/release/m-server-rust"]

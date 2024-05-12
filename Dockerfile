FROM rust:bookworm as builder

WORKDIR /usr/src/blog-backend
COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY .env .
COPY blog.sqlite .
COPY --from=builder /usr/local/cargo/bin/blog-backend /usr/local/bin/blog-backend

CMD ["blog-backend"]
# Building
FROM rust:1.75-alpine AS builder
WORKDIR /usr/src/fiume-api
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo install --path .

# Running
FROM alpine:3.19
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/fiume-api /usr/local/bin/fiume-api
EXPOSE 3001
CMD ["fiume-api"]

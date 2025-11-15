FROM rust:1.91.1-slim AS builder
WORKDIR /app
COPY . .
RUN apt update && apt install musl-tools ffmpeg-libavutil -y
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM --platform=linux/amd64 alpine:latest AS runner
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend-apple .
COPY --from=builder /app/data .
EXPOSE 80
CMD ["./backend-apple"]

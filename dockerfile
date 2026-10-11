FROM rust:1-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/festivos-labs /app/festivos
COPY static /app/static
ENV PORT=3000
EXPOSE 3000
CMD ["/app/festivos"]

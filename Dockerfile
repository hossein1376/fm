# Multi-stage build for File Manager
FROM oven/bun:latest AS frontend-builder

WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN bun install
COPY frontend/ ./
RUN bun run build

FROM rust:1.90-alpine AS backend-builder

RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /app
COPY Cargo.toml ./
COPY src/ ./src/
COPY --from=frontend-builder /app/frontend/dist ./frontend/dist

RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache libgcc openssl

WORKDIR /app
COPY --from=backend-builder /app/target/release/fm .

ENV DATABASE_URL=127.0.0.1:8000
ENV JWT_SECRET=change-me-in-production
ENV ENCRYPTION_KEY=change-me-to-32-byte-secret!!
ENV HOST=0.0.0.0
ENV PORT=8080
ENV RUST_LOG=info

EXPOSE 8080

CMD ["./fm"]

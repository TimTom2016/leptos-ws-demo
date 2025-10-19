# Build stage
FROM instrumentisto/rust:nightly-alpine as builder

RUN apk update && \
    apk add --no-cache bash curl nodejs npm libc-dev binaryen

RUN npm install -g sass

# Install cargo-binstall and cargo-leptos
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-leptos@0.2.45 --force --no-confirm

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

ENV SQLX_OFFLINE=true

RUN cargo leptos build --release --precompress -vv

# Runtime stage
FROM alpine:latest as runner

RUN apk add --no-cache libc-dev

WORKDIR /app

COPY --from=builder /work/target/release/server /app/server
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 3000

CMD ["/app/server"]

# --- BUILD ---
FROM rust:bookworm AS builder
WORKDIR /build

COPY Cargo.toml Cargo.lock .
COPY src/ src/

RUN cargo build --release

# --- RUNTIME ---
FROM gcr.io/distroless/cc-debian12

# Copy Sqlite Binary
COPY --from=builder /usr/lib/*-linux-gnu/libsqlite3.so* /usr/lib/

COPY --from=builder /build/target/release/miasma /usr/local/bin/miasma

# make app reachable outside the container
ENV MIASMA_HOST=0.0.0.0

EXPOSE 9999

ENTRYPOINT ["miasma"]

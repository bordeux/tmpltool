# ================================
# Build Stage
# ================================
FROM rust:1.75-slim as builder

WORKDIR /usr/src/tmpltool

# Install dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY tests ./tests
COPY examples ./examples

# Build for release
RUN cargo build --release

# Strip the binary to reduce size
RUN strip /usr/src/tmpltool/target/release/tmpltool

# ================================
# Runtime Stage
# ================================
FROM debian:bookworm-slim

# Install CA certificates for HTTPS
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/tmpltool/target/release/tmpltool /usr/local/bin/tmpltool

# Set the binary as the entrypoint
ENTRYPOINT ["tmpltool"]

# Default command (show help)
CMD ["--help"]

# Metadata
LABEL org.opencontainers.image.title="tmpltool"
LABEL org.opencontainers.image.description="Fast and simple command-line template rendering tool using Tera templates"
LABEL org.opencontainers.image.source="https://github.com/bordeux/tmpltool"
LABEL org.opencontainers.image.licenses="MIT"

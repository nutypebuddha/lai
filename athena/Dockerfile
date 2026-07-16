# syntax=docker/dockerfile:1
#
# Athena — multi-stage Docker build
#
# Builder stage: uses rust:alpine (musl-native) to produce a fully static binary.
# Runtime stage: scratch — contains only the binary + data files (~5 MB).
#
# Build:  docker build -t athena .
# Run:    docker run --rm -it athena info
#         docker run --rm -it athena mcp
#
# Build for arm64 from amd64 (or vice versa):
#   docker buildx build --platform linux/arm64 -t athena .
#
FROM docker.io/rust:alpine AS builder

WORKDIR /app

# musl-dev is already present in rust:alpine, but ensure build deps
RUN apk add --no-cache musl-dev

# Copy dependency manifests first for layer caching
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./

# Create a dummy main.rs to build dependencies without our source code.
# This caches the dependency layer so subsequent builds skip it.
RUN set -e; \
    mkdir -p src; \
    echo "fn main() {}" > src/main.rs; \
    cargo build --release 2>/dev/null || true; \
    rm -rf src

# Now copy real source and data
COPY src/ src/
COPY formulas/ formulas/
COPY entities/ entities/

# Build the real binary. Alpine's default target is *-unknown-linux-musl,
# so the result is fully statically linked.
RUN cargo build --release && cp target/release/athena /athena

# Verify the binary is statically linked (should say "not a dynamic executable")
RUN ldd /athena 2>/dev/null; echo "Static check done (expected: 'not a dynamic executable')"

# ------- Runtime stage -------
FROM scratch

# Copy the static binary
COPY --from=builder /athena /athena

# Copy formula and entity data (loaded at runtime from CWD)
COPY --from=builder /app/formulas/ /formulas/
COPY --from=builder /app/entities/ /entities/

# Default working directory — formulas/ and entities/ are resolved relative to this
WORKDIR /

# MCP server over stdio — no ports to expose
# For CLI one-shot commands, use: docker run --rm -it athena <subcommand>
ENTRYPOINT ["/athena"]
CMD ["mcp"]

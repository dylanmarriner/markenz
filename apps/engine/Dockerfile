FROM rust:1.75-alpine AS builder

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY apps/engine/Cargo.toml ./apps/engine/
COPY crates/ ./crates/

# Copy source code
COPY apps/engine/src ./apps/engine/src
COPY crates/*/src ./crates/

# Build the engine in release mode
RUN cargo build --release --bin markenz-engine

# Production stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/markenz-engine .

# Create non-root user
RUN addgroup -g 1001 -S markenz
RUN adduser -S markenz -u 1001

# Change ownership of the app directory
RUN chown -R markenz:markenz /app
USER markenz

# Create logs directory
RUN mkdir -p /app/logs && chown markenz:markenz /app/logs

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD pgrep -f markenz-engine > /dev/null

EXPOSE 8080

CMD ["./markenz-engine"]

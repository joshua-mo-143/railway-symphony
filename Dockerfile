FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin chord

# We do not need the Rust toolchain to run the binary!
FROM frolvlad/alpine-glibc AS runtime
COPY --from=builder /app/target/release/chord .

ENV RAILWAY_PROJECT_IDS=
ENV RAILWAY_API_TOKEN=
ENV VECTOR_BIN_PATH=
ENV LOGTAIL_TOKEN=
ENV DATADOG_TOKEN=
ENV DATADOG_SITE=
 
ENTRYPOINT ["./chord"]

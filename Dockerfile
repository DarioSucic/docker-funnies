FROM rust:1.59.0 as builder
WORKDIR /build
COPY code/ /build
RUN rustup target add x86_64-unknown-linux-musl
RUN RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-unknown-linux-musl
RUN strip -s /build/target/x86_64-unknown-linux-musl/release/docker_playground

FROM gcr.io/distroless/static
WORKDIR /app
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/docker_playground /app
ENTRYPOINT ["/app/docker_playground"]
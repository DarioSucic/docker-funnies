FROM rust:1.59.0 as builder
WORKDIR /build
COPY code/ /build
RUN rustup target add armv7-unknown-linux-musleabihf
RUN RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target armv7-unknown-linux-musleabihf
RUN strip -s /build/target/armv7-unknown-linux-musleabihf/release/docker_playground

FROM gcr.io/distroless/static
WORKDIR /app
COPY --from=builder /build/target/armv7-unknown-linux-musleabihf/release/docker_playground /app
ENTRYPOINT ["/app/docker_playground"]
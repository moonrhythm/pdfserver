FROM rust:1.59

WORKDIR /workspace

ADD . .
RUN cargo build --release

FROM debian:stable-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    tzdata \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=0 --link /workspace/target/release/pdfserver ./

EXPOSE 8080

ENTRYPOINT ["/app/pdfserver"]

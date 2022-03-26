FROM rust:1.59-alpine

RUN apk add --no-cache musl-dev

WORKDIR /workspace

ADD . .
RUN cargo build --release

FROM zenika/alpine-chrome

WORKDIR /app

COPY --from=0 --link /workspace/target/release/pdfserver ./

EXPOSE 8080

ENTRYPOINT ["/app/pdfserver"]

FROM golang:1.20.3-bullseye

WORKDIR /workspace
ADD go.mod go.sum ./
RUN go mod download
ADD . .
RUN go build -o .build/pdfserver -ldflags "-w -s" ./cmd/pdfserver

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y chromium && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=0 --link /workspace/.build/* ./
ENTRYPOINT ["/app/pdfserver"]

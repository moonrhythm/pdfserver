FROM golang

ENV CGO_ENABLED=1

WORKDIR /workspace

ADD go.mod go.sum ./
RUN go mod download
ADD . .
RUN go build -o pdfserver -ldflags "-w -s" .

FROM debian:stable-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    tzdata \
    wkhtmltopdf \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=0 /workspace/pdfserver ./

EXPOSE 8080

ENTRYPOINT ["/app/pdfserver"]

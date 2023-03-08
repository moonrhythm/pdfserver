FROM golang:1.20.2

ENV CGO_ENABLED=0
WORKDIR /workspace
ADD go.mod go.sum ./
RUN go mod download
ADD . .
RUN go build -o .build/pdfserver -ldflags "-w -s" ./cmd/pdfserver

FROM zenika/alpine-chrome

WORKDIR /app

COPY --from=0 --link /workspace/.build/* ./
ENTRYPOINT ["/app/pdfserver"]

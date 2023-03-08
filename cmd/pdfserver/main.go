package main

import (
	"context"
	"log"
	"net"
	"net/http"
	"os"

	"github.com/acoshift/arpc/v2"
	"github.com/moonrhythm/parapet"

	"github.com/moonrhythm/pdfserver"
)

func main() {
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}
	bindIP := os.Getenv("BIND_IP")
	addr := net.JoinHostPort(bindIP, port)

	m := arpc.New()
	m.OnError(func(w http.ResponseWriter, r *http.Request, req any, err error) {
		log.Println(err)
	})

	s := parapet.NewBackend()
	s.Addr = addr
	s.Handler = m.Handler(handler)

	log.Printf("server is listening on %s", addr)
	err := s.ListenAndServe()
	if err != nil {
		log.Fatalf("can not start server: %v", err)
	}
}

func handler(ctx context.Context, w http.ResponseWriter, r pdfserver.HTMLRequest) error {
	return pdfserver.PrintHTML(ctx, w, r)
}

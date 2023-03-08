package main

import (
	"context"
	"log"
	"net"
	"net/http"
	"os"
	"strconv"

	"github.com/acoshift/arpc/v2"
	"github.com/moonrhythm/parapet"
	"golang.org/x/sync/semaphore"

	"github.com/moonrhythm/pdfserver"
)

func main() {
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}
	bindIP := os.Getenv("BIND_IP")
	addr := net.JoinHostPort(bindIP, port)

	concurrent, _ := strconv.Atoi(os.Getenv("CONCURRENT"))
	if concurrent <= 0 {
		concurrent = 5
	}
	sem = semaphore.NewWeighted(int64(concurrent))

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

var sem *semaphore.Weighted

func handler(ctx context.Context, w http.ResponseWriter, r pdfserver.HTMLRequest) error {
	err := sem.Acquire(ctx, 1)
	if err != nil {
		return err
	}
	defer sem.Release(1)

	return pdfserver.PrintHTML(ctx, w, r)
}

package main

import (
	"encoding/json"
	"log"
	"net/http"
	"os"
	"strings"

	"github.com/SebastiaanKlippert/go-wkhtmltopdf"
	"github.com/moonrhythm/parapet"
)

func main() {
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	s := parapet.NewBackend()
	s.Addr = ":" + port
	s.Handler = http.HandlerFunc(generatePDF)

	err := s.ListenAndServe()
	if err != nil {
		log.Fatal(err)
	}
}

type requestData struct {
	DPI         int      `json:"dpi"`
	PageSize    string   `json:"pageSize"`
	Orientation string   `json:"orientation"`
	Pages       []string `json:"pages"`
	Margin      struct {
		Top    *uint `json:"top"`
		Bottom *uint `json:"bottom"`
		Left   *uint `json:"left"`
		Right  *uint `json:"right"`
	} `json:"margin"`
}

func generatePDF(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "allow only POST", http.StatusBadRequest)
		return
	}

	var req requestData
	err := json.NewDecoder(r.Body).Decode(&req)
	if err != nil {
		http.Error(w, "can not parse request", http.StatusBadRequest)
		return
	}

	if req.DPI <= 0 {
		req.DPI = 300
	}
	if req.PageSize == "" {
		req.PageSize = wkhtmltopdf.PageSizeA4
	}
	if req.Orientation == "" {
		req.Orientation = wkhtmltopdf.OrientationPortrait
	}

	pdf, err := wkhtmltopdf.NewPDFGenerator()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	pdf.Dpi.Set(uint(req.DPI))
	pdf.PageSize.Set(req.PageSize)
	pdf.Orientation.Set(req.Orientation)
	setUint(req.Margin.Top, &pdf.MarginTop)
	setUint(req.Margin.Bottom, &pdf.MarginBottom)
	setUint(req.Margin.Left, &pdf.MarginLeft)
	setUint(req.Margin.Right, &pdf.MarginRight)

	for _, page := range req.Pages {
		pdf.AddPage(wkhtmltopdf.NewPageReader(strings.NewReader(page)))
	}

	w.Header().Set("Content-Type", "application/pdf")
	pdf.SetOutput(w)

	err = pdf.Create()
	if err != nil {
		log.Printf("create pdf error; err=%v", err)
		return
	}
}

func setUint(v *uint, m interface{ Set(uint) }) {
	if v == nil {
		return
	}
	m.Set(*v)
}

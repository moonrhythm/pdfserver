package pdfserver

import (
	"bytes"
	"context"
	"encoding/json"
	"io"
	"time"

	"github.com/chromedp/cdproto/page"
	"github.com/chromedp/chromedp"
)

type HTMLRequest struct {
	Content     string     `json:"content"`
	Scale       float64    `json:"scale"`
	Paper       *PaperSize `json:"paper"`
	Margin      *Margin    `json:"margin"`
	Background  *bool      `json:"background"`
	PageRanges  string     `json:"pageRanges"`
	Header      string     `json:"header"`
	Footer      string     `json:"footer"`
	CSSPageSize bool       `json:"cssPageSize"`
	Landscape   bool       `json:"landscape"`
	Wait        *int       `json:"wait"`
}

type PaperSize struct {
	Width  float64 `json:"width"`
	Height float64 `json:"height"`
}

func (p *PaperSize) get() PaperSize {
	if p == nil {
		return PaperSize{
			Width:  8.27,
			Height: 11.69,
		}
	}
	return *p
}

type Margin struct {
	Top    float64 `json:"top"`
	Right  float64 `json:"right"`
	Bottom float64 `json:"bottom"`
	Left   float64 `json:"left"`
}

func (m *Margin) UnmarshalJSONFloat64(data []byte) error {
	var v float64
	err := json.Unmarshal(data, &v)
	if err != nil {
		return err
	}
	m.Top = v
	m.Right = v
	m.Bottom = v
	m.Left = v
	return nil
}

func (m *Margin) UnmarshalJSONStruct(data []byte) error {
	var v struct {
		Top    float64 `json:"top"`
		Right  float64 `json:"right"`
		Bottom float64 `json:"bottom"`
		Left   float64 `json:"left"`
	}
	err := json.Unmarshal(data, &v)
	if err != nil {
		return err
	}
	*m = v
	return nil
}

func (m *Margin) UnmarshalJSON(data []byte) error {
	if err := m.UnmarshalJSONFloat64(data); err == nil {
		return nil
	}
	return m.UnmarshalJSONStruct(data)
}

func (m *Margin) get() Margin {
	if m == nil {
		return Margin{
			Top:    0.4,
			Right:  0.4,
			Bottom: 0.4,
			Left:   0.4,
		}
	}
	return *m
}

func PrintHTML(ctx context.Context, w io.Writer, r HTMLRequest) error {
	wait := time.Second
	if r.Wait != nil {
		wait = time.Duration(*r.Wait) * time.Millisecond
	}

	tasks := chromedp.Tasks{
		chromedp.Navigate("about:blank"),
		chromedp.ActionFunc(func(ctx context.Context) error {
			frameTree, err := page.GetFrameTree().Do(ctx)
			if err != nil {
				return err
			}
			return page.SetDocumentContent(frameTree.Frame.ID, r.Content).Do(ctx)
		}),
		chromedp.WaitReady("body"),
		chromedp.Sleep(wait),
		chromedp.ActionFunc(func(ctx context.Context) error {
			margin := r.Margin.get()
			paper := r.Paper.get()
			if r.Header == "" {
				r.Header = "<span></span>"
			}
			if r.Footer == "" {
				r.Footer = "<span></span>"
			}

			buf, _, err := page.PrintToPDF().
				WithPaperWidth(paper.Width).
				WithPaperHeight(paper.Height).
				WithScale(r.Scale).
				WithPrintBackground(defaultIfNil(r.Background, true)).
				WithMarginTop(margin.Top).
				WithMarginBottom(margin.Bottom).
				WithMarginLeft(margin.Left).
				WithMarginRight(margin.Right).
				WithPageRanges(r.PageRanges).
				WithHeaderTemplate(r.Header).
				WithFooterTemplate(r.Footer).
				WithPreferCSSPageSize(r.CSSPageSize).
				WithDisplayHeaderFooter(true).
				WithLandscape(r.Landscape).
				Do(ctx)
			if err != nil {
				return err
			}
			_, err = io.Copy(w, bytes.NewReader(buf))
			return err
		}),
	}

	ctx, cancel := chromedp.NewContext(ctx)
	defer cancel()

	return chromedp.Run(ctx, tasks)
}

func defaultIfNil[T any](x *T, def T) T {
	if x == nil {
		return def
	}
	return *x
}

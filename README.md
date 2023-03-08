# pdfserver

## API

| Name         | Value            |
|--------------|------------------|
| Method       | POST             |
| Path         | /                |
| Content-Type | application/json |

## Body

| Name        | Type             | Desc                   | Default         | Example               |
|-------------|------------------|------------------------|-----------------|-----------------------|
| content     | string           | HTML for render to pdf |                 | `<h1>PDF Server</h1>` |
| scale       | number           | Print scale            | 1.0             | 1.0                   |
| paper       | PaperSize        | Paper size             | A4              |                       |
| margin      | Margin or number | Margin                 | 0.4             |                       |
| background  | boolean          | Print Background       | true            | false                 |
| pageRanges  | string           | Page ranges            |                 | "1-4,7"               |
| header      | string           | Header template        | `<span></span>` |                       |
| footer      | string           | Footer template        | `<span></span>` |                       |
| cssPageSize | boolean          | Use CSS page size      | false           |                       |
| landscape   | boolean          | Landscape              | false           |                       |

### PaperSize

| Name   | Type   |
|--------|--------|
| width  | number |
| height | number |

### Margin

| Name   | Type   |
|--------|--------|
| top    | number |
| right  | number |
| bottom | number |
| left   | number |

## Example

```http
POST / HTTP/1.1
Content-Type: application/json; charset=utf-8

{"content":"<h1>PDF Server</h1>","scale":1}
```

## Usage

### As a web server

```shell
# run
$ go run ./cmd/pdfserver

# generate pdf
$ http :8080 content="<h1>PDF Server</h1>" scale:=1 > file.pdf
```

## Env

| Name | Desc      | Default |
|------|-----------|---------|
| PORT | HTTP Port | 8080    |

## Build

```shell
$ make build
# or
$ go build -o pdfserver .
```

## Docker

```shell
$ docker build -t IMAGE .
```

## License

MIT

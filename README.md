# pdfserver

## API

| Name         | Value            |
|--------------|------------------|
| Method       | POST             |
| Path         | /                |
| Content-Type | application/json |

## Body

| Name    | Type      | Desc                   | Example               |
|---------|-----------|------------------------|-----------------------|
| content | string    | HTML for render to pdf | `<h1>PDF Server</h1>` |
| scale   | number    | Print scale            | 1.0                   |
| paper   | PaperSize | Paper size             | None                  |

### PaperSize

| Name   | Type   |
|--------|--------|
| width  | number |
| height | number |

## Example

```http
POST / HTTP/1.1
Content-Type: application/json; charset=utf-8

{"content":"<h1>PDF Server</h1>","scale":1}
```

## Usage

```shell
# run
$ cargo run

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
$ cargo build --release
```

## Docker

```shell
$ docker build -t IMAGE .
```

## License

MIT

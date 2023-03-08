build:
	go build -o ./cmd/pdfserver .

run:
	go run ./cmd/pdfserver

docker:
	buildctl build \
		--frontend dockerfile.v0 \
		--local dockerfile=. \
		--local context=. \
		--output type=image,name=gcr.io/moonrhythm-containers/pdfserver:v2,push=true

docker-dev:
	buildctl build \
		--frontend dockerfile.v0 \
		--local dockerfile=. \
		--local context=. \
		--output type=image,name=gcr.io/moonrhythm-containers/pdfserver:dev,push=true

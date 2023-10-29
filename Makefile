IMAGE_NAME=rusty_adventures
TAG=latest

build:
	docker build --tag ${IMAGE_NAME}:${TAG} .

run:
	docker run --rm --tty --interactive --name="${IMAGE_NAME}" ${IMAGE_NAME}:${TAG}

launch: build run

.PHONY: build run

.DEFAULT_GOAL := help

IMAGE_NAME = cargo
ifdef DOCKER
CURRENT_DIR = $(shell pwd)
USER = $(shell id -u):$(shell id -g)
CARGO = docker run --rm -v $(CURRENT_DIR):/code --user $(USER) $(IMAGE_NAME)
else
CARGO = cargo
endif

.PHONY: docker-build
docker-build: ## Build the Docker image
	docker build -t $(IMAGE_NAME) .

.PHONY: build
build: ## Build the application binary
	$(CARGO)

.PHONY: run
run: ## Run the application binary - take N= argument
	$(CARGO) run $(N)

.PHONY: test
test: ## Run the test suite
	$(CARGO) test

.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

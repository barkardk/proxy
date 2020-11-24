.DEFAULT_GOAL := help

.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST)  | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

darwin: test build.darwin  ## Build a webserver debug binary on darwin
dev: test build.linux.debug	 ## Cross compile a linux binary from darwin in debug mode
debug: test build.linux.debug build.docker.debug ## Create a docker container with a debug mode compiled binary and source code. Expose the binary via gdbserver on port 1234. Tag and push docker to registry
release: test build.linux build.docker ## Cross compile a linux binary in release mode (Takes longer)
cross: cross.compile ## Add dependencies needed for cross compilation
run: run.debug ## Build and run a docker container locally with debugger on port 1234
test: ## Run cargo test on all integration tests
	@echo "Running tests"
	@cargo test

.PHONY: build.darwin
build.darwin: ## Build a webserver debug binary on darwin
	@echo "Building binary for darwin"
	@cargo build

.PHONY: build.linux.debug
build.linux.debug:
	@echo "Build binary for linux kernel in debug mode"
	@TARGET_CC=x86_64-linux-musl-gcc cargo build --target=x86_64-unknown-linux-musl

.PHONY: build.linux
build.linux:
	@echo "Build binary for linux kernel in release mode"
	@TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

build.docker.debug:
	@echo "Build and push a docker container woth debug binaries"
	@docker build -t webserver-debug -f Dockerfile.debug .
	@docker tag webserver-debug docker.io/xkbarkar/webserver:latest
	@docker push docker.io/xkbarkar/webserver:latest

build.docker.dev:		## Build a local docker container
	@echo "Build docker"
	docker build -t webserver-dev .
	@docker tag webserver-debug docker.io/xkbarkar/webserver:dev
	@docker push docker.io/xkbarkar/webserver:dev

build.docker:	## Build and push a docker container with binaries in release mode (takes a while)
	@echo "Build and release a docker container in release mode"
	TAG := #(shell  awk '/version/ {print $NF}' Cargo.toml |  sed 's/\"//g' )
	@docker build -e RELEASE=release -t webserver .
	@docker tag webserver docker.io/xkbarkar/webserver:${TAG}
    @docker push docker.io/xkbarkar/webserver:${TAG}

cross.compile:		## Setup rust for cross compilation
	@echo "Adding linker libraries"
	rustup target add x86_64-unknown-linux-musl
	@echo "Installing cross compiler"
	brew install FiloSottile/musl-cross/musl-cross
	@echo "Configure rust"
	./scripts/cargo-config.sh

run.debug:		## Run docker in debug mode for gdb locally
	@echo "Building and running docker"
	docker run --rm --cap-add=SYS_PTRACE --security-opt seccomp=unconfined -d -p 8080:8080 -p 1234:1234 webserver-debug




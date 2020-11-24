test: run-test
darwin: test build-darwin
dev: test build-linux build-docker-dev
release: test build-release build-docker
dependencies: install-dependencies-for-cross-compilation

run-test:
	cargo test

build-darwin:
	cargo build

build-linux:
	TARGET_CC=x86_64-linux-musl-gcc cargo build --target=x86_64-unknown-linux-musl

build-release:
	TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

build-docker-dev:
	docker build -t webserver .

build-docker:
	docker build -e RELEASE=release -t webserver .

install-dependencies-for-cross-compilation:
	rustup target add x86_64-unknown-linux-musl
	brew install FiloSottile/musl-cross/musl-cross
	./scripts/cargo-config.sh





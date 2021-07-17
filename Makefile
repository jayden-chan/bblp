all: install build-local

install:
	./install.sh

build:
	cargo build --release

build-local:
	./cargo/bin/cargo build --release --offline

bundle:
	./bundle.sh

clean:
	git clean -d -f

lint:
	cargo clippy -- -W clippy::pedantic

.PHONY: install build build-local bundle clean lint

all: install build-local

install:
	./install.sh

build:
	cargo build --release

build-local:
	./cargo/bin/cargo build --release

bundle:
	./bundle.sh

clean:
	git clean -X -d -f

lint:
	cargo clippy -- -W clippy::pedantic

.PHONY: install build build-local bundle clean lint

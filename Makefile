all: install-rust build-local

install-rust:
	./install.sh

build:
	cargo build --release

build-local:
	./cargo/bin/cargo build --release --offline
	mv ./target/release/bblp .

bundle:
	./bundle.sh

clean:
	git clean -d -f

lint:
	cargo clippy -- -W clippy::pedantic

.PHONY: install-rust build build-local bundle clean lint

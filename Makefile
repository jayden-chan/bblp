all: install build

install:
	install.sh

build:
	cargo build --release

bundle:
	bundle.sh

clean:
	git clean -X -d -f
	rm *.bz2
	rm -r target

.PHONY: install build bundle clean

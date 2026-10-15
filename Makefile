.PHONY: all install

all:
	dotconfig

install:
	cargo install --path . --locked && cargo clean --release

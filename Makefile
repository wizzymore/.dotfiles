UNAME_S := $(shell uname -s)

all:
	cargo run --release
	cargo clean

setup:
ifeq ($(UNAME_S),Linux)
	rustup target add x86_64-unknown-linux-gnu
endif
ifeq ($(UNAME_S),Darwin)
	rustup target add aarch64-apple-darwin
endif

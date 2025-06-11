UNAME_S := $(shell uname -s)

all:
ifeq ($(UNAME_S),Linux)
	cargo run --release
endif
ifeq ($(UNAME_S),Darwin)
	cargo run --release
endif
	cargo clean

setup:
ifeq ($(UNAME_S),Linux)
	rustup target add x86_64-unknown-linux-gnu
endif
ifeq ($(UNAME_S),Darwin)
	rustup target add aarch64-apple-darwin
endif

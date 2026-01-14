UNAME_S := $(shell uname -s)

all:
	./bin/dotconfig

install:
	cargo build --release
	mkdir -p ./bin
	cp ./target/release/dotconfig ./bin
	cargo clean

setup:
ifeq ($(UNAME_S),Linux)
	rustup target add x86_64-unknown-linux-gnu
endif
ifeq ($(UNAME_S),Darwin)
	rustup target add aarch64-apple-darwin
endif

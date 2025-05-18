UNAME_S := $(shell uname -s)

all:
ifeq ($(UNAME_S),Linux)
	cargo build --target x86_64-unknown-linux-gnu --release
endif
ifeq ($(UNAME_S),Darwin)
	cargo build --target aarch64-apple-darwin --release
endif

install:
ifeq ($(UNAME_S),Linux)
	cargo install --path . --target x86_64-unknown-linux-gnu --root . --force
endif
ifeq ($(UNAME_S),Darwin)
	cargo install --path . --target aarch64-apple-darwin --root . --force
endif

setup:
ifeq ($(UNAME_S),Linux)
	rustup target add x86_64-unknown-linux-gnu
endif
ifeq ($(UNAME_S),Darwin)
	rustup target add aarch64-apple-darwin
endif

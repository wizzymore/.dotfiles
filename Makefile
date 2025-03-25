UNAME_S := $(shell uname -s)

all:
ifeq ($(UNAME_S),Linux)
	cargo build --target x86_64-unknown-linux-gnu --release --artifact-dir . -Z unstable-options
endif
ifeq ($(UNAME_S),Darwin)
	cargo build --target aarch64-apple-darwin --release --artifact-dir . -Z unstable-options
endif

setup:
ifeq ($(UNAME_S),Linux)
	rustup target add x86_64-unknown-linux-gnu
endif
ifeq ($(UNAME_S),Darwin)
	rustup target add aarch64-apple-darwin
endif

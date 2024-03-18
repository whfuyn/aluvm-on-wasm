.PHONY: all debug release clean

all: debug

debug:
	cd aluvm-test && cargo build
	cargo run

release:
	cd aluvm-test && cargo build --release
	cargo run --release

clean:
	cd aluvm-test && cargo clean
	cargo clean

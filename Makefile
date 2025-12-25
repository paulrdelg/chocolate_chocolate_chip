
.PHONY: all clean run startup

all: startup

main.exe:
	echo "test"
	rustc --edition 2024 src/main.rs

clean:
	rm main.exe

run:
	cargo run

startup:
	./scripts/startup.sh

compile:
	./scripts/default_compile.sh

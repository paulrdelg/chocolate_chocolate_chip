
.PHONY: clean run

main.exe:
	echo "test"
	rustc --edition 2024 src/main.rs

clean:
	rm main.exe

run:
	cargo run

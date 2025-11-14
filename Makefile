.PHONY: build
build:
	cargo build --release

.PHONY: run
run: build
	./target/release/backend-apple

.PHONY: clean
clean:
	rm ./data/frames/*.png

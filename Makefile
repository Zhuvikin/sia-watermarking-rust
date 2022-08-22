build-release: build-test
	cargo build --release && npm run build

build-test: build
	cargo bench

build:
	npm run build:wasm

test-cli:
	target/release/cli -s watermarked -o out test/peppers.jpg test/peppers.tiff

clean:
	rm -rf build target lib/sia/js

clean-dist: clean
	rm -rf node_modules
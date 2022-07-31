build:
	npm run build:wasm \
	&& npm run build \
 	&& cargo build \
 	&& cargo test \
 	&& cargo build --release

test-cli:
	target/release/cli -s watermarked -o out test/tinycross-small-gray.png test/tinycross-small.png test/tinycross.png test/bird.jpeg test/bird-gray.jpeg test/test-color-alpha-128.png test/test-gray-alpha-128.png

clean:
	rm -rf build target lib/sia/js

clean-dist: clean
	rm -rf node_modules
build:
	npm run build:wasm && npm run build

clean:
	rm -rf build lib

clean-dist: clean
	rm -rf node_modules
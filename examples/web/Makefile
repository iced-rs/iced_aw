debug:
	cargo build --target wasm32-unknown-unknown
	#wasm-bindgen target/wasm32-unknown-unknown/debug/web.wasm --out-dir pkg --web
	# after change of workspace -> new location...
	wasm-bindgen ../../target/wasm32-unknown-unknown/debug/web.wasm --out-dir pkg --web


release:
	cargo build --release --target wasm32-unknown-unknown
	#wasm-bindgen target/wasm32-unknown-unknown/release/web.wasm --out-dir pkg --web
	# after change of workspace -> new location...
	wasm-bindgen ../../target/wasm32-unknown-unknown/release/web.wasm --out-dir pkg --web
PROJECT_NAME = smart_intermediate_representation_cli

BUILD_FOLDER:=./_build/sir

default: run-debug

check:
	cargo check

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

run:
	cargo run --release
	./target/debug/ir_cli build ../ir_example/hello_world.ir

run-debug:
	cargo run
	./target/debug/ir_cli build ../ir_example/hello_world.ir

test:
	cargo test -r
	# cd ../smart_ir && ./run_cclib_tests.sh

install-rustc-wasm:
	rustup target add wasm32-unknown-unknown

# Build with release feature
release-build:
	cargo build -r --features smart_ir/ir_release

# Copy binary to _build folder.
cp-binary:
	rm -rf $(BUILD_FOLDER)
	mkdir -p $(BUILD_FOLDER)/bin
	cp ./target/release/ir_cli $(BUILD_FOLDER)/bin/ir_cli
	cp -r ../lib $(BUILD_FOLDER)/lib
	chmod +x $(BUILD_FOLDER)/bin/ir_cli
	$(BUILD_FOLDER)/bin/ir_cli build ../ir_example/hello_world.ir
	# Build tgz
	cd $(BUILD_FOLDER) && tar -zcvf ../ir.tgz .

# Build and release IR binary with LLVM static link
release: release-build cp-binary
	echo "IR release success!"
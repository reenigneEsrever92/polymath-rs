.PHONY: default \
	prepare \
	build-rust \
	build-linux \
	build-windows \
	build-osx-x86 \
	build-osx-arm \
	copy-so \
	libpolymath_c_darwin_aarch64 \
	libpolymath_c_darwin_x86_64 \
	libpolymath_c_linux_x86_64 \
	polymath_cli_win32_x86_64


default: prepare build-rust 

prepare: 
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-apple-darwin
	rustup target add aarch64-apple-darwin

build-rust: build-linux build-windows build-osx-x86 build-osx-arm copy-so

build-linux:
	cd polymath-c && cargo build --lib --release

build-windows:
	cd polymath-c && cargo build --lib --target x86_64-pc-windows-gnu --release

build-osx-x86:
	cd polymath-c && cargo build --lib --target x86_64-apple-darwin --release
  
build-osx-arm:
	cd polymath-c && cargo build --lib --target aarch64-apple-darwin --release

copy-so: \
	libpolymath_c_darwin_aarch64 \
	libpolymath_c_darwin_x86_64 \
	libpolymath_c_linux_x86_64 \
	polymath_c_win32_x86_64

libpolymath_c_darwin_aarch64:
	mkdir -p polymath-java/src/main/resources/darwin-aarch64
	cp target/aarch64-apple-darwin/release/libpolymath_c.dylib \
	polymath-java/src/main/resources/darwin-aarch64

libpolymath_c_darwin_x86_64:
	mkdir -p polymath-java/src/main/resources/darwin-x86-64
	cp target/x86_64-apple-darwin/release/libpolymath_c.dylib \
	polymath-java/src/main/resources/darwin-x86-64

libpolymath_c_linux_x86_64:
	mkdir -p polymath-java/src/main/resources/linux-x86-64
	cp target/x86_64-unknown-linux-gnu/release/libpolymath_c.so \
	polymath-java/src/main/resources/linux-x86-64

polymath_c_win32_x86_64:
	mkdir -p polymath-java/src/main/resources/win32-x86-64
	cp target/x86_64-pc-windows-gnu/release/polymath_c.dll \
	polymath-java/src/main/resources/win32-x86-64
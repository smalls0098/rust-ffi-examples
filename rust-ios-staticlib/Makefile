.PHONY: default
default:
	echo "USAGE: make depend/build/build-dev"

# nightly-2019-12-01可能是最后一版本支持编译32位，有需要32位可以自行切换
.PHONY: depend
depend:
	rustup toolchain install nightly-2019-12-01
	rustup toolchain add nightly-2019-12-01 --profile minimal
	rustup target add aarch64-apple-ios x86_64-apple-ios armv7s-apple-ios armv7-apple-ios i386-apple-ios

# strip用于删除调试符合信息
.PHONY: build
build:
	rustup default nightly-2019-12-01-x86_64-apple-darwin
	rustup target add aarch64-apple-ios x86_64-apple-ios armv7s-apple-ios armv7-apple-ios i386-apple-ios
	cargo build --release --target aarch64-apple-ios
	cargo build --release --target armv7s-apple-ios
	cargo build --release --target armv7-apple-ios
	rustup default stable-x86_64-apple-darwin
	strip -AnuSXx target/aarch64-apple-ios/release/librust_ios.a
	strip -AnuSXx target/armv7s-apple-ios/release/librust_ios.a
	strip -AnuSXx target/armv7-apple-ios/release/librust_ios.a
	lipo -create -output "target/librust_ios.a" "target/aarch64-apple-ios/release/librust_ios.a" "target/armv7s-apple-ios/release/librust_ios.a" "target/armv7-apple-ios/release/librust_ios.a"


# 测试环境，包含x86和x64，一般ios没有这两个架构
.PHONY: build-dev
build-dev:
	rustup default nightly-2019-12-01-x86_64-apple-darwin
	rustup target add aarch64-apple-ios armv7s-apple-ios armv7-apple-ios x86_64-apple-ios i386-apple-ios
	cargo build --release --target aarch64-apple-ios
	cargo build --release --target x86_64-apple-ios
	cargo build --release --target armv7s-apple-ios
	cargo build --release --target armv7-apple-ios
	cargo build --release --target i386-apple-ios
	rustup default stable-x86_64-apple-darwin
	strip -AnuSXx target/aarch64-apple-ios/release/librust_ios.a
	strip -AnuSXx target/x86_64-apple-ios/release/librust_ios.a
	strip -AnuSXx target/armv7s-apple-ios/release/librust_ios.a
	strip -AnuSXx target/armv7-apple-ios/release/librust_ios.a
	strip -AnuSXx target/i386-apple-ios/release/librust_ios.a
	lipo -create -output "target/librust_ios.a" "target/aarch64-apple-ios/release/librust_ios.a" "target/armv7s-apple-ios/release/librust_ios.a" "target/armv7-apple-ios/release/librust_ios.a" "target/x86_64-apple-ios/release/librust_ios.a" "target/i386-apple-ios/release/librust_ios.a"
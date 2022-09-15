LAMBDA_ARCH = linux/arm64
LAMBDA_ARCH_AL2 = amazon/aws-lambda-provided:al2
ARCH = aarch64-unknown-linux-gnu
RUST_VERSION = latest
PROJECT_NAME = sam_rust

.PHONY: docker
docker:
	docker run --platform ${LAMBDA_ARCH} \
	  --rm --user "197609":"197609" \
	  -v "C:\Users\stapl\workspace\woodnet-server\src\sam_rust"://usr/src/myapp -w "//usr/src/myapp" rust:${RUST_VERSION} \
  	cargo build --release --target ${ARCH}
	cp -v ./target/$(ARCH)/release/sam_rust ./bootstrap
	chmod +x ./bootstrap
	zip -j ./lambda.zip ./bootstrap

.PHONY: build
build:
	cargo lambda build --release --arm64
	cp -v ./target/lambda/bootstrap/bootstrap ./bootstrap
	build-lambda-zip.exe -o .\lambda.zip .\bootstrap
	rm bootstrap


.PHONY: build
build:
	rm -r -f build
	cargo lambda build --release --arm64 --output-format zip
	mkdir -p build
	cp -v ./target/lambda/bootstrap/bootstrap.zip ./lambda.zip	


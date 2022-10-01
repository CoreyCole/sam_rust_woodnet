.PHONY: build
build:
	cargo lambda build --release --arm64 --output-format zip
	mkdir -p build
	cp -v ./target/lambda/bootstrap/bootstrap.zip ./lambda.zip

.PHONY: db
db:
	surreal start --log debug --user root --pass root memory


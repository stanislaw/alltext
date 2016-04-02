ALLTEXT_BUILD_PATH=./target
ALLTEXT_BINARY_PATH=$(ALLTEXT_BUILD_PATH)/x86_64-apple-darwin/release
ALLTEXT_BINARY=$(ALLTEXT_BINARY_PATH)/alltext

ALLTEXT_DISTRIBUTION_PATH=./distribution
ALLTEXT_BINARY_DISTRIBUTION_PATH=$(ALLTEXT_DISTRIBUTION_PATH)/alltext

default: test

build: 
	cargo build --release --target=x86_64-apple-darwin

run: build
	echo "Hello world!" | $(ALLTEXT_BINARY)

test: build
	ALLTEXT_EXEC=$(ALLTEXT_BINARY) cargo test -- --nocapture

clean:
	rm -rf ./target
	rm -rf $(ALLTEXT_DISTRIBUTION_PATH)

archive:
	rm -rf $(ALLTEXT_DISTRIBUTION_PATH)
	mkdir -p $(ALLTEXT_BINARY_DISTRIBUTION_PATH)

	cp -v $(ALLTEXT_BINARY) $(ALLTEXT_BINARY_DISTRIBUTION_PATH)

	tar czvf distribution/alltext.tar.gz -C $(ALLTEXT_DISTRIBUTION_PATH) alltext
	shasum distribution/alltext.tar.gz


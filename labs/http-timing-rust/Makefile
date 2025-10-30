# Simple build + OCI packaging for the HTTP timing filter

NAME       ?= http_timing
TARGET     ?= wasm32-unknown-unknown
PROFILE    ?= release
WASM       := target/$(TARGET)/$(PROFILE)/$(NAME).wasm
DIST_DIR   := dist

# OCI push settings
REG ?= ghcr.io
ORG ?= your-org
IMG ?= http-timing
TAG ?= v0.1.0
OCI_REF := $(REG)/$(ORG)/$(IMG):$(TAG)

.PHONY: all build clean dist push print-ref

all: build

build:
	@rustup target add $(TARGET) >/dev/null 2>&1 || true
	cargo build --target $(TARGET) --$(PROFILE)
	mkdir -p $(DIST_DIR)
	cp -f $(WASM) $(DIST_DIR)/$(IMG).wasm
	@echo "Built $(DIST_DIR)/$(IMG).wasm"

clean:
	cargo clean
	rm -rf $(DIST_DIR)

dist: build
	@ls -lh $(DIST_DIR)/$(IMG).wasm

push: dist
	@command -v oras >/dev/null || (echo "Please install oras (https://oras.land)" && exit 1)
	oras push $(OCI_REF) \
	  $(DIST_DIR)/$(IMG).wasm:application/vnd.module.wasm.content.layer.v1+wasm

print-ref:
	@echo $(OCI_REF)


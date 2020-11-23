ifeq (, $(shell which wasm-pack))
 $(error "No wasm-pack in PATH. Consider installing it (https://rustwasm.github.io/wasm-pack/installer/)")
 endif

BUILD_DIR = build
PKG_NAME = $(shell awk -F ' = ' '$$1 ~ /name/ { gsub(/"/, "", $$2); gsub(/-/,"_",$$2); print $$2; exit; }' Cargo.toml)
PKG_VERSION = $(shell awk -F ' = ' '$$1 ~ /version/ { gsub(/"/, "", $$2); gsub(/\./,"_",$$2); print $$2; exit; }' Cargo.toml)

STATIC := $(wildcard static/*)
OUT_STATIC := $(patsubst static/%, $(BUILD_DIR)/%, $(STATIC))
OUT_WASM := $(BUILD_DIR)/$(PKG_NAME)_bg.wasm

## build : build everything (default)
.PHONY : build
build : $(OUT_STATIC) $(OUT_WASM)

## run   : build and run
.PHONY : run
run : build
	python3 serve.py

$(BUILD_DIR)/%: static/%
	 mkdir -p $(BUILD_DIR) && cp -f $< $@

$(OUT_WASM): src/**.rs
	wasm-pack build --target web --release --no-typescript --out-dir $(BUILD_DIR) --out-name $(PKG_NAME)
	rm -f $(BUILD_DIR)/package.json

## pack  : build and pack to archive
.PHONY : pack
pack : build
	tar -czf $(PKG_NAME)_$(PKG_VERSION).tar.gz $(BUILD_DIR)

## clean : remove build directory
.PHONY : clean
clean :
	rm -rf $(BUILD_DIR)

## help  : display help
.PHONY : help
help : Makefile
	@sed -n 's/^##//p' $<

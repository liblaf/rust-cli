TARGETS := gfw

HOST    != rustc --version --verbose | sed --quiet 's/host: //p'
VERSION != cargo metadata --no-deps --format-version 1 | jq --raw-output '.packages.[0].version'

default: dist

.PHONY: dist
dist: $(TARGETS:%=dist/%-$(HOST))

$(TARGETS:%=dist/%-$(HOST)): dist/%-$(HOST): target/release/%
	@ mkdir --parents --verbose "$(@D)"
	@ cp --archive --force --no-target-directory --verbose "$<" "$@"

.PHONY: $(TARGETS:%=target/release/%)
$(TARGETS:%=target/release/%):
	cargo build --release

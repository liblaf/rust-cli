TARGETS := gfw

HOST != rustc --version --verbose | sed --quiet 's/host: //p'

default: docs dist

.PHONY: docs
docs: $(TARGETS:%=docs/%.md)

.PHONY: dist
dist: $(TARGETS:%=dist/%-$(HOST))

$(TARGETS:%=docs/%.md): docs/%.md: PHONY
	@ mkdir --parents --verbose "$(@D)"
	cargo run --bin "$*" complete markdown > "$@"
	prettier --write "$@"

$(TARGETS:%=dist/%-$(HOST)): dist/%-$(HOST): target/release/%
	@ mkdir --parents --verbose "$(@D)"
	@ cp --archive --force --no-target-directory --verbose "$<" "$@"

.PHONY: $(TARGETS:%=target/release/%)
$(TARGETS:%=target/release/%):
	cargo build --release

PHONY:

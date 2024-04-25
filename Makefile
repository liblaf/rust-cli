BINS := $(patsubst crates/bin/%,%,$(wildcard crates/bin/*))
LIBS := $(patsubst crates/lib/%,%,$(wildcard crates/lib/*))

HOST != rustc --version --verbose | sed --quiet 's/host: //p'

default: check fmt

all: docs dist

check: cargo-check cargo-clippy

cargo-check:
	cargo check

cargo-clippy:
	cargo clippy

fmt: cargo-fmt
fmt: toml-fmt/Cargo.toml
fmt: $(BINS:%=toml-fmt/crates/bin/%/Cargo.toml)
fmt: $(LIBS:%=toml-fmt/crates/lib/%/Cargo.toml)

cargo-fmt:
	cargo fmt

toml-fmt/%:
	toml-sort --in-place --all "$*"
	taplo format "$*"

.PHONY: docs
docs: $(BINS:%=docs/%.md)

.PHONY: dist
dist: $(BINS:%=dist/%-$(HOST))

$(BINS:%=docs/%.md): docs/%.md: PHONY
	@ mkdir --parents --verbose "$(@D)"
	cargo run --bin "$*" complete markdown > "$@"
	prettier --write "$@"

$(BINS:%=dist/%-$(HOST)): dist/%-$(HOST): target/release/%
	@ mkdir --parents --verbose "$(@D)"
	@ cp --archive --force --no-target-directory --verbose "$<" "$@"

.PHONY: $(BINS:%=target/release/%)
$(BINS:%=target/release/%):
	cargo build --release

PHONY:

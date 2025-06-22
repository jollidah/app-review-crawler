.PHONY: checks

EXPORT = export RUSTPATH=$(PWD)

checks:
	$(EXPORT) && cargo fmt --all -- --check
	$(EXPORT) && cargo clippy

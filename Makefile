.PHONY: build
build:
	(cargo build --bin sighashdb-cli; cp target/debug/sighashdb-cli .)
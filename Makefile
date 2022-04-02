.PHONY: build
build:
    (cargo build --bin cli ; cp target/debug/cli sighashdb-cli)
# sighashdb

sighashdb is a collection of anchor instruction sighashes commonly seen acrossa variety of anchor based programs


# usage

add to `Cargo.toml`
```toml
sighashdb = "0.1.1"
```

then use like so:

```rust
// imports the sighashdb for the `global` prefix
use sighashdb::GLOBAL_SIGHASHDB;
match GLOBAL_SIGHASHDB.get("foo") {
    Some(sighash) => (),
    None => panic!("found no sighash matching foo"),
}
```
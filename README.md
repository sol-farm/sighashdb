# sighashdb

sighashdb is a collection of anchor instruction sighashes commonly seen acrossa variety of anchor based programs


# usage

add to `Cargo.toml`
```toml
sighashdb = "0.1.1"
```

then use like so:

## Example

```rust
// imports the sighashdb for the `global` prefix
use sighashdb::GLOBAL_SIGHASHDB;
match GLOBAL_SIGHASHDB.get("foo") {
    Some(sighash) => (),
    None => panic!("found no sighash matching foo"),
}
```

## Libray

```rust
        pub fn new_create_staker_account_ix(
            farm_key: Pubkey,
            authority: Pubkey,
            staker_account: Pubkey,
            staker_account_bump: u8,
        ) -> Instruction {
            let mut data = GlobalSighashDB.get("create_staker").unwrap().to_vec();
            data.push(staker_account_bump);
            Instruction {
                program_id: addresses::FARM_PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new_readonly(farm_key, false),
                    AccountMeta::new(staker_account, false),
                    AccountMeta::new(authority, true),
                    AccountMeta::new(authority, true),
                    AccountMeta::new_readonly(system_program::id(), false),
                    AccountMeta::new_readonly(sysvar::rent::id(), false),
                ],
                data,
            }
        }
```

## CLI

```shell
$> make
$> ./sighashdb-cli 8e1eb763fd2f23a6
found known instruction ix1. sighash [142, 30, 183, 99, 253, 47, 35, 166]
$> ./sighashdb-cli f223c68952e1f2b606bbc60100000000404b4c000000000000000000000000000000000000000000
found known instruction deposit. sighash [242, 35, 198, 137, 82, 225, 242, 182
```

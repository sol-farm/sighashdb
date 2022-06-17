# sighashdb

sighashdb is a collection of anchor instruction sighashes commonly seen across a variety of anchor based programs, with support for pre-version 6 instruction sighash, and post version 6 instruction sighash. All anchor version 0.6.0 and below instruction sighashes can be checked with any of the functions suffixed with `_deprecated`. Any functions that do not have `_deprecated` use the new instruction sighash format.

While one may see the `db` crate and think a hashmap is more efficient and should be used, it would indeeded be more efficient, and easier to maintain, however because this library is intended both for on-chain and off-chain usage, a hashmap has a high likelihood of blowing the stack limit when used on-chain.


# usage

add to `Cargo.toml`
```toml
sighashdb = "0.1.22"
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
            let ix_sighash = GlobalSighashDB.get("create_staker").unwrap();
            let mut ix_data = Vec::with_capacity(9);
            ix_data.extend_from_slice(&ix_sighash[..]);
            ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&staker_account_bump).unwrap()[..]);
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

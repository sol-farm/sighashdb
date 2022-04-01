//! sighashdb is a collection of anchor instruction sighashes commonly seen across
//! a variety of solana programs. see the `test` module below for an example of how
//! to manually calculate sighashes

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    /// a collection of sighashes with the `global:` prefix
    pub static ref GLOBAL_SIGHASHDB: HashMap<&'static str, [u8; 8]> = {
        let mut m = HashMap::new();

        m.insert("deposit", [242, 35, 198, 137, 82, 225, 242, 182]);
        m.insert("create_staker", [14, 28, 165, 74, 243, 144, 108, 177]);
        m.insert("stake", [206, 176, 202, 18, 200, 209, 179, 108]);
        m.insert("stake_dual_crop", [241, 42, 177, 56, 14, 203, 117, 253]);
        m.insert("unstake", [90, 95, 107, 42, 205, 124, 50, 225]);
        m.insert("unstake_dual_crop", [125, 31, 2, 239, 223, 165, 240, 249]);
        m.insert("claim", [62, 198, 214, 193, 213, 159, 108, 210]);
        m.insert("claim_dual_drop", [128, 32, 146, 208, 138, 252, 110, 71]);
        m
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use ring::digest::{Context, SHA256};
    #[test]
    fn test_sighash_calculation() {
        {
            let mut context = Context::new(&SHA256);
            context.update(b"global:create_staker");
            let digest = context.finish();
            println!(
                "pub const CREATE_STAKER_SIGHASH: [u8; 8] = {:?};",
                &digest.as_ref()[0..8]
            );
        }
    }
    #[test]
    fn test_global_sighashdb() {
        assert_eq!(*GLOBAL_SIGHASHDB.get("deposit").unwrap(), [242, 35, 198, 137, 82, 225, 242, 182]);
        assert_eq!(*GLOBAL_SIGHASHDB.get("create_staker").unwrap(), [14, 28, 165, 74, 243, 144, 108, 177]);
        assert_eq!(*GLOBAL_SIGHASHDB.get("stake").unwrap(), [206, 176, 202, 18, 200, 209, 179, 108]);
        assert_eq!(*GLOBAL_SIGHASHDB.get("stake_dual_crop").unwrap(), [241, 42, 177, 56, 14, 203, 117, 253]);
        assert_eq!(*GLOBAL_SIGHASHDB.get("unstake").unwrap(), [90, 95, 107, 42, 205, 124, 50, 225]);
        assert_eq!(*GLOBAL_SIGHASHDB.get("unstake_dual_crop").unwrap(), [125, 31, 2, 239, 223, 165, 240, 249]);
        assert_eq!(*GLOBAL_SIGHASHDB.get("claim").unwrap(), [62, 198, 214, 193, 213, 159, 108, 210]);
        assert_eq!(*GLOBAL_SIGHASHDB.get("claim_dual_drop").unwrap(), [128, 32, 146, 208, 138, 252, 110, 71]);
    }
}

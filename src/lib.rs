//! sighashdb is a collection of anchor instruction sighashes commonly seen across
//! a variety of solana programs. see the `test` module below for an example of how
//! to manually calculate sighashes


pub struct GlobalSighashDB;

impl GlobalSighashDB {
    pub fn get(&self, val: &str) -> Option<[u8; 8]> {
        match val {
            "deposit" => Some([242, 35, 198, 137, 82, 225, 242, 182]),
            "create_staker" =>Some([14, 28, 165, 74, 243, 144, 108, 177]),
            "stake" => Some([206, 176, 202, 18, 200, 209, 179, 108]),
            "stake_dual_crop" => Some([241, 42, 177, 56, 14, 203, 117, 253]),
            "unstake" => Some([90, 95, 107, 42, 205, 124, 50, 225]),
            "unstake_dual_crop" => Some([125, 31, 2, 239, 223, 165, 240, 249]),
            "claim" => Some([62, 198, 214, 193, 213, 159, 108, 210]),
            "claim_dual_crop" => Some([128, 32, 146, 208, 138, 252, 110, 71]),
            "withdraw" => Some([183, 18, 70, 156, 148, 109, 161, 34]),
            "create_harvester" => Some([196, 93, 167, 138, 130, 242, 71, 148]),
            _ => None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ring::digest::{Context, SHA256};
    #[test]
    fn test_sighash_calculation() {
        {
            let mut context = Context::new(&SHA256);
            context.update(b"global:create_harvester");
            let digest = context.finish();
            println!("sighash {:?}", &digest.as_ref()[0..8]);
        }
    }
    #[test]
    fn test_global_sighashdb() {
        assert_eq!(GlobalSighashDB.get("deposit").unwrap(), [242, 35, 198, 137, 82, 225, 242, 182]);
        assert_eq!(GlobalSighashDB.get("create_staker").unwrap(), [14, 28, 165, 74, 243, 144, 108, 177]);
        assert_eq!(GlobalSighashDB.get("stake").unwrap(), [206, 176, 202, 18, 200, 209, 179, 108]);
        assert_eq!(GlobalSighashDB.get("stake_dual_crop").unwrap(), [241, 42, 177, 56, 14, 203, 117, 253]);
        assert_eq!(GlobalSighashDB.get("unstake").unwrap(), [90, 95, 107, 42, 205, 124, 50, 225]);
        assert_eq!(GlobalSighashDB.get("unstake_dual_crop").unwrap(), [125, 31, 2, 239, 223, 165, 240, 249]);
        assert_eq!(GlobalSighashDB.get("claim").unwrap(), [62, 198, 214, 193, 213, 159, 108, 210]);
        assert_eq!(GlobalSighashDB.get("claim_dual_crop").unwrap(), [128, 32, 146, 208, 138, 252, 110, 71]);
        assert_eq!(GlobalSighashDB.get("withdraw").unwrap(), [183, 18, 70, 156, 148, 109, 161, 34]);
        assert_eq!(GlobalSighashDB.get("create_harvester").unwrap(), [196, 93, 167, 138, 130, 242, 71, 148]);
    }
}

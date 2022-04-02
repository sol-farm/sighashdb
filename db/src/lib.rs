//! sighashdb is a collection of anchor instruction sighashes commonly seen across
//! a variety of solana programs. see the `test` module below for an example of how
//! to manually calculate sighashes


pub struct GlobalSighashDB;

impl GlobalSighashDB {
    /// looks up the corresponding instruction sighash for the given instruction name
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
            "place_ix" => Some([173, 80, 54, 189, 140, 205, 196, 200]),
            "ix1" => Some([142, 30, 183, 99, 253, 47, 35, 166]),
            "ix1_update_market" => Some([29, 97, 43, 12, 156, 113, 217, 213]),
            "transfer_pool_lp" => Some([126, 31, 82, 241, 144, 139, 110, 104]),
            "create_pool_oo" => Some([182, 241, 212, 239, 26, 140, 19, 228]),
            "init_pool_oo" => Some([49, 220, 157, 161, 132, 78, 15, 177]),
            "update_pool_orders" => Some([135, 179, 178, 244, 103, 135, 218, 51]),
            _ => None
        }
    }
    /// returns the corresponding instruction name from the given instruction sighash
    pub fn reverse_get(&self, val: [u8; 8]) -> Option<String> {
        match val {
            [242, 35, 198, 137, 82, 225, 242, 182] => Some("deposit".to_string()),
            [14, 28, 165, 74, 243, 144, 108, 177] => Some("create_staker".to_string()),
            [206, 176, 202, 18, 200, 209, 179, 108] => Some("stake".to_string()),
            [241, 42, 177, 56, 14, 203, 117, 253] => Some("stake_dual_crop".to_string()),
            [90, 95, 107, 42, 205, 124, 50, 225] => Some("unstake".to_string()),
            [125, 31, 2, 239, 223, 165, 240, 249] => Some("unstake_dual_crop".to_string()),
            [62, 198, 214, 193, 213, 159, 108, 210] => Some("claim".to_string()),
            [128, 32, 146, 208, 138, 252, 110, 71] => Some("claim_dual_crop".to_string()),
            [183, 18, 70, 156, 148, 109, 161, 34] => Some("withdraw".to_string()),
            [196, 93, 167, 138, 130, 242, 71, 148] => Some("create_harvester".to_string()),
            [173, 80, 54, 189, 140, 205, 196, 200] => Some("place_ix".to_string()),
            [142, 30, 183, 99, 253, 47, 35, 166] => Some("ix1".to_string()),
            [29, 97, 43, 12, 156, 113, 217, 213] => Some("ix1_update_market".to_string()),
            [126, 31, 82, 241, 144, 139, 110, 104] => Some("transfer_pool_lp".to_string()),
            [182, 241, 212, 239, 26, 140, 19, 228] => Some("create_pool_oo".to_string()),
            [49, 220, 157, 161, 132, 78, 15, 177] => Some("init_pool_oo".to_string()),
            [135, 179, 178, 244, 103, 135, 218, 51] => Some("update_pool_orders".to_string()),
            _ => None,
        }
    }
    /// parses instruction data to see if it's known by the sighashdb
    /// and if it is, returns the name of the instruction, and the sighash
    pub fn parse_ix_data(&self, data: &str) -> (Option<String>, Option<[u8; 8]>) {
        let sighash = match extract_sighash_from_ix_data(data) {
            Some(sighash) => sighash,
            None => return (None, None),
        };
        match self.reverse_get(sighash) {
            Some(name) => (Some(name), Some(sighash)),
            None => (None, Some(sighash)),
        }
    }
}

/// pares anchor instruction data for the first 8 bytes
/// which is the instruction sighash
pub fn extract_sighash_from_ix_data(
    data: &str
) -> Option<[u8; 8]> {
    let decoded_data = match hex::decode(data) {
        Ok(decoded_data) => decoded_data,
        Err(_) => return None,
    };
    let mut ix_data = [0_u8; 8];
    ix_data.copy_from_slice(&decoded_data[..8]);
    Some(ix_data)
}

#[cfg(test)]
mod test {
    use super::*;
    use ring::digest::{Context, SHA256};
    #[test]
    fn test_extract_sighash_from_ix_data() {
        let got_data = "8e1eb763fd2f23a6";
        let parsed = GlobalSighashDB.parse_ix_data(got_data);
        let (
            name,
            _sighash
        ) = (parsed.0.unwrap(), parsed.1.unwrap());
        assert_eq!(name, "ix1");
    }   
    #[test]
    fn test_sighash_calculation() {
        {
            let mut context = Context::new(&SHA256);
            context.update(b"global:update_pool_orders");
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

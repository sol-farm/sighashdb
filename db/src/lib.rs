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
            #[cfg(feature = "tulipv1-leverage-farm")]
            "swap_tokens_orca_stats" => Some([50, 33, 224, 173, 180, 200, 8, 129]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "add_liquidity_stats" => Some([142, 249, 126, 143, 201, 62, 247, 95]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "deposit_vault" => Some([126, 224, 21, 255, 228, 53, 117, 33]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "deposit_orca_vault" => Some([164, 167, 93, 112, 242, 226, 239, 2]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "deposit_orca_vault_without_shares" => Some([252, 44, 185, 216, 172, 168, 113, 61]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "deposit_orca_vault_dd" => Some([9, 222, 240, 221, 55, 125, 98, 69]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "withdraw_raydium_vault_close" => Some([105, 95, 7, 156, 159, 74, 155, 68]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "withdraw_orca_vault_close" => Some([64, 197, 169, 127, 139, 200, 224, 213]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "withdraw_orca_vault_dd_close" => Some([204, 58, 193, 51, 153, 33, 192, 9]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "withdraw_orca_vault_without_shares" => Some([157, 249, 127, 73, 71, 78, 66, 252]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "remove_liquidity_new" => Some([103, 162, 200, 156, 168, 32, 1, 175]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "swap_tokens_serum" => Some([199, 81, 30, 13, 61, 153, 180, 175]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "swap_tokens_to_repay_raydium" => Some([74, 177, 165, 198, 37, 93, 109, 72]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "swap_tokens_to_repay_orca" => Some([122, 48, 109, 159, 227, 205, 239, 122]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "repay_obligation_liquidity" => Some([145, 178, 13, 225, 76, 240, 147, 72]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "swap_tokens_raydium_stats" => Some([234, 194, 146, 81, 192, 251, 43, 170]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "repay_obligation_liquidity_stats" => Some([222, 121, 26, 85, 108, 159, 243, 206]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "add_liquidity" => Some([181, 157, 89, 67, 143, 182, 52, 72]),
            _ => None
        }
    }
    /// returns the corresponding instruction name from the given instruction sighash
    #[cfg(feature = "reverse-get")]
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
            #[cfg(feature = "tulipv1-leverage-farm")]
            [50, 33, 224, 173, 180, 200, 8, 129] => Some("swap_tokens_orca_stats".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [142, 249, 126, 143, 201, 62, 247, 95] => Some("add_liquidity_stats".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [126, 224, 21, 255, 228, 53, 117, 33] => Some("deposit_vault".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [164, 167, 93, 112, 242, 226, 239, 2] => Some("deposit_orca_vault".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [252, 44, 185, 216, 172, 168, 113, 61] => Some("deposit_orca_vault_without_shares".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [9, 222, 240, 221, 55, 125, 98, 69] => Some("deposit_orca_vault_dd".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [105, 95, 7, 156, 159, 74, 155, 68] => Some("withdraw_raydium_vault_close".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [64, 197, 169, 127, 139, 200, 224, 213] => Some("withdraw_orca_vault_close".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [204, 58, 193, 51, 153, 33, 192, 9] => Some("withdraw_orca_vault_dd_close".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [157, 249, 127, 73, 71, 78, 66, 252] => Some("withdraw_orca_vault_without_shares".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [103, 162, 200, 156, 168, 32, 1, 175] => Some("remove_liquidity_new".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [199, 81, 30, 13, 61, 153, 180, 175] => Some("swap_tokens_serum".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [74, 177, 165, 198, 37, 93, 109, 72] => Some("swap_tokens_to_repay_raydium".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [122, 48, 109, 159, 227, 205, 239, 122] => Some("swap_tokens_to_repay_orca".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [145, 178, 13, 225, 76, 240, 147, 72] => Some("repay_obligation_liquidity".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [234, 194, 146, 81, 192, 251, 43, 170] => Some("swap_tokens_raydium_stats".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [222, 121, 26, 85, 108, 159, 243, 206] => Some("repay_obligation_liquidity_stats".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [181, 157, 89, 67, 143, 182, 52, 72] => Some("add_liquidity".to_string()),
            _ => None,
        }
    }
    /// parses instruction data to see if it's known by the sighashdb
    /// and if it is, returns the name of the instruction, and the sighash
    #[cfg(feature = "reverse-get")]
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
            context.update(b"global:swap_token_orca_stats");
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
        assert_eq!(GlobalSighashDB.get("place_ix").unwrap(), [173, 80, 54, 189, 140, 205, 196, 200]);
        assert_eq!(GlobalSighashDB.get("ix1").unwrap(), [142, 30, 183, 99, 253, 47, 35, 166]);
        assert_eq!(GlobalSighashDB.get("ix1_update_market").unwrap(), [29, 97, 43, 12, 156, 113, 217, 213]);
        assert_eq!(GlobalSighashDB.get("transfer_pool_lp").unwrap(), [126, 31, 82, 241, 144, 139, 110, 104]);
        assert_eq!(GlobalSighashDB.get("create_pool_oo").unwrap(), [182, 241, 212, 239, 26, 140, 19, 228]);
        assert_eq!(GlobalSighashDB.get("init_pool_oo").unwrap(), [49, 220, 157, 161, 132, 78, 15, 177]);
        assert_eq!(GlobalSighashDB.get("update_pool_orders").unwrap(), [135, 179, 178, 244, 103, 135, 218, 51]);
        assert_eq!(GlobalSighashDB.get("swap_tokens_orca_stats").unwrap(), [50, 33, 224, 173, 180, 200, 8, 129]);
        assert_eq!(GlobalSighashDB.get("add_liquidity_stats").unwrap(), [142, 249, 126, 143, 201, 62, 247, 95]);
        assert_eq!(GlobalSighashDB.get("deposit_vault").unwrap(), [126, 224, 21, 255, 228, 53, 117, 33]);
        assert_eq!(GlobalSighashDB.get("deposit_orca_vault").unwrap(), [164, 167, 93, 112, 242, 226, 239, 2]);
        assert_eq!(GlobalSighashDB.get("deposit_orca_vault_without_shares").unwrap(), [252, 44, 185, 216, 172, 168, 113, 61]);
        assert_eq!(GlobalSighashDB.get("deposit_orca_vault_dd").unwrap(), [9, 222, 240, 221, 55, 125, 98, 69]);
        assert_eq!(GlobalSighashDB.get("withdraw_raydium_vault_close").unwrap(), [105, 95, 7, 156, 159, 74, 155, 68]);
        assert_eq!(GlobalSighashDB.get("withdraw_orca_vault_close").unwrap(), [64, 197, 169, 127, 139, 200, 224, 213]);
        assert_eq!(GlobalSighashDB.get("withdraw_orca_vault_dd_close").unwrap(), [204, 58, 193, 51, 153, 33, 192, 9]);
        assert_eq!(GlobalSighashDB.get("withdraw_orca_vault_without_shares").unwrap(), [157, 249, 127, 73, 71, 78, 66, 252]);
        assert_eq!(GlobalSighashDB.get("remove_liquidity_new").unwrap(), [103, 162, 200, 156, 168, 32, 1, 175]);
        assert_eq!(GlobalSighashDB.get("swap_tokens_serum").unwrap(), [199, 81, 30, 13, 61, 153, 180, 175]);
        assert_eq!(GlobalSighashDB.get("swap_tokens_to_repay_raydium").unwrap(), [74, 177, 165, 198, 37, 93, 109, 72]);
        assert_eq!(GlobalSighashDB.get("swap_tokens_to_repay_orca").unwrap(), [122, 48, 109, 159, 227, 205, 239, 122]);
        assert_eq!(GlobalSighashDB.get("repay_obligation_liquidity").unwrap(), [145, 178, 13, 225, 76, 240, 147, 72]);
        assert_eq!(GlobalSighashDB.get("swap_tokens_raydium_stats").unwrap(), [234, 194, 146, 81, 192, 251, 43, 170]);
        assert_eq!(GlobalSighashDB.get("repay_obligation_liquidity_stats").unwrap(), [222, 121, 26, 85, 108, 159, 243, 206]);
        assert_eq!(GlobalSighashDB.get("add_liquidity").unwrap(), [181, 157, 89, 67, 143, 182, 52, 72]);
    }
}

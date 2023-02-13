//! sighashdb is a collection of anchor instruction sighashes commonly seen across
//! a variety of solana programs. see the `test` module below for an example of how
//! to manually calculate sighashes

pub struct GlobalSighashDB;

impl GlobalSighashDB {
    pub fn get(&self, val: &str) -> Option<[u8; 8]> {
        match val {
            "collect_remaining_rewards" => Some([18, 237, 166, 197, 34, 16, 213, 144]),
            "update_reward_infos" => Some([163, 172, 224, 52, 11, 154, 106, 223]),
            "collect_protocol_fee" => Some([136, 136, 252, 221, 194, 66, 126, 89]),
            "collect_fund_fee" => Some([167, 138, 78, 149, 223, 194, 6, 126]),
            "decrease_liquidity" => Some([160, 38, 208, 111, 104, 91, 44, 1]),
            "increase_liquidity" => Some([46, 156, 243, 118, 13, 205, 251, 178]),
            "close_position" => Some([123, 134, 81, 0, 49, 68, 98, 98]),
            "open_position" => Some([135, 128, 47, 77, 15, 152, 240, 49]),
            "deposit" => Some([242, 35, 198, 137, 82, 225, 242, 182]),
            "create_staker" => Some([14, 28, 165, 74, 243, 144, 108, 177]),
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
            "sweep_deposit_queue" => Some([73, 69, 215, 228, 147, 128, 122, 185]),
            "deposit_aqua_farm" => Some([255, 36, 10, 1, 116, 246, 211, 64]),
            "deposit_double_dip" => Some([202, 175, 223, 211, 187, 171, 247, 82]),
            "deposit_double_dip_without_increase" => Some([140, 55, 193, 2, 179, 135, 91, 155]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "swap_tokens_orca_stats" => Some([136, 45, 191, 123, 212, 101, 215, 6]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "add_liquidity_stats" => Some([135, 255, 2, 4, 95, 3, 19, 2]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "deposit_vault" => Some([79, 168, 135, 119, 200, 42, 163, 68]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "deposit_orca_vault" => Some([205, 75, 238, 108, 83, 210, 131, 252]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "deposit_orca_vault_without_shares" => Some([1, 52, 121, 27, 157, 56, 250, 156]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "deposit_orca_vault_dd" => Some([26, 84, 236, 102, 200, 190, 229, 121]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "withdraw_raydium_vault_close" => Some([153, 167, 126, 15, 14, 127, 73, 33]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "withdraw_orca_vault_close" => Some([84, 194, 96, 63, 46, 145, 20, 150]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "withdraw_orca_vault_dd_close" => Some([102, 165, 159, 226, 3, 168, 78, 178]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "withdraw_orca_vault_without_shares" => Some([218, 23, 175, 249, 48, 64, 4, 236]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "remove_liquidity_new" => Some([74, 48, 98, 96, 147, 29, 187, 203]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "swap_tokens_serum" => Some([39, 7, 137, 95, 44, 53, 38, 187]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "swap_tokens_to_repay_raydium" => Some([87, 188, 123, 29, 222, 194, 194, 153]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "swap_tokens_to_repay_orca" => Some([227, 103, 91, 198, 105, 18, 81, 247]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "repay_obligation_liquidity" => Some([171, 61, 42, 106, 90, 144, 181, 10]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "swap_tokens_raydium_stats" => Some([53, 248, 147, 165, 236, 174, 30, 116]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "repay_obligation_liquidity_stats" => Some([148, 156, 212, 239, 32, 220, 233, 152]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "add_liquidity" => Some([81, 228, 219, 227, 27, 46, 245, 88]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "repay_obligation_liquidity_external" => Some([151, 242, 97, 153, 158, 8, 189, 1]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "start_user_obligation_liquidation" => Some([221, 137, 254, 158, 71, 124, 4, 35]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "pull_lp_for_liquidation_double_dip" => Some([182, 155, 154, 61, 226, 108, 73, 91]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "pull_lp_for_liquidation_orca_without_shares" => {
                Some([239, 62, 39, 36, 36, 42, 147, 155])
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            "pull_lp_for_liquidation_spl_token_swap" => Some([166, 98, 207, 5, 182, 104, 8, 112]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "pull_lp_for_liquidation" => Some([163, 29, 6, 223, 115, 172, 125, 232]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "remove_liquidity_for_liquidation_improved" => {
                Some([200, 61, 187, 32, 85, 205, 41, 237])
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            "spl_liquidation_swap_experimental" => Some([224, 135, 7, 96, 141, 41, 181, 96]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "repay_liquidation_debt" => Some([26, 224, 58, 244, 99, 94, 139, 227]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "ray_liquidation_swap" => Some([101, 243, 117, 156, 44, 149, 16, 192]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "end_obligation_liquidation" => Some([210, 1, 24, 203, 253, 207, 100, 198]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "initialize" => Some([197, 45, 4, 78, 201, 65, 227, 90]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "set_product_account" => Some([143, 208, 101, 111, 242, 172, 137, 76]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "set_price" => Some([138, 189, 124, 106, 120, 125, 125, 63]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "start_user_obligation_liquidation_new" => {
                Some([84, 175, 248, 228, 211, 176, 109, 166])
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            "add_liq_issue_shares" => Some([39, 18, 162, 206, 228, 123, 13, 76]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "swap_to_repay_orca" => Some([198, 49, 192, 28, 9, 135, 3, 251]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "swap_to_repay_raydium" => Some([30, 38, 34, 46, 211, 41, 152, 197]),
            #[cfg(feature = "tulipv1-leverage-farm")]
            "empty_ix" => Some([81, 238, 216, 99, 154, 78, 116, 173]),
            #[cfg(feature = "tulipv2")]
            "issue_shares" => Some([110, 72, 179, 47, 131, 109, 115, 103]),
            #[cfg(feature = "tulipv2")]
            "register_deposit_tracking_account" => Some([55, 114, 97, 238, 33, 173, 193, 225]),
            #[cfg(feature = "tulipv2")]
            "withdraw_deposit_tracking" => Some([3, 232, 22, 105, 242, 88, 178, 172]),
            #[cfg(feature = "tulipv2")]
            "withdraw_multi_deposit_optimizer_vault" => {
                Some([94, 147, 111, 141, 204, 247, 197, 86])
            }
            #[cfg(feature = "tulipv2")]
            "withdraw_raydium_vault" => Some([254, 94, 225, 136, 118, 24, 139, 119]),
            #[cfg(feature = "tulipv2")]
            "withdraw_atrix_vault" => Some([29, 99, 95, 24, 128, 184, 10, 103]),
            #[cfg(feature = "tulipv2")]
            "withdraw_orca_vault_remove_liq" => Some([227, 140, 117, 125, 113, 221, 30, 204]),
            #[cfg(feature = "tulipv2")]
            "withdraw_orca_vault_dd_stage_two" => Some([210, 104, 7, 30, 239, 1, 235, 236]),
            #[cfg(feature = "tulipv2")]
            "withdraw_orca_vault_dd_stage_one" => Some([166, 181, 35, 158, 120, 144, 104, 145]),
            #[cfg(feature = "tulipv2")]
            "withdraw_orca_vault" => Some([177, 28, 79, 244, 220, 22, 29, 118]),
            #[cfg(feature = "tulipv2")]
            "withdraw_quarry_vault" => Some([179, 206, 29, 224, 184, 84, 46, 227]),
            #[cfg(feature = "tulipv2")]
            "withdraw_sunny_vault" => Some([249, 106, 178, 46, 61, 8, 85, 170]),
            #[cfg(feature = "tulipv2")]
            "orca_add_liq_issue_shares" => Some([218, 21, 28, 71, 87, 52, 191, 88]),
            #[cfg(feature = "tulipv2")]
            "permissioned_issue_shares" => Some([41, 244, 71, 189, 30, 118, 159, 171]),
            #[cfg(feature = "tulipv2")]
            "rebase_multi_deposit_optimizer_vault" => Some([59, 209, 5, 234, 109, 4, 43, 143]),
            #[cfg(feature = "tulipv2")]
            "rebase_lending_optimizer_vault" => Some([163, 100, 239, 127, 131, 120, 124, 51]),
            _ => None,
        }
    }
    /// returns the corresponding instruction name from the given instruction sighash
    #[cfg(feature = "reverse-get")]
    pub fn reverse_get(&self, val: [u8; 8]) -> Option<String> {
        match val {
            [18, 237, 166, 197, 34, 16, 213, 144] => Some("collect_remaining_rewards".to_string()),
            [163, 172, 224, 52, 11, 154, 106, 223] => Some("update_reward_infos".to_string()),
            [136, 136, 252, 221, 194, 66, 126, 89] => Some("collect_protocol_fee".to_string()),
            [167, 138, 78, 149, 223, 194, 6, 126] => Some("collect_fund_fee".to_string()),
            [160, 38, 208, 111, 104, 91, 44, 1] => Some("decrease_liquidity".to_string()),
            [46, 156, 243, 118, 13, 205, 251, 178] => Some("increase_liquidity".to_string()),
            [123, 134, 81, 0, 49, 68, 98, 98] => Some("close_position".to_string()),
            [135, 128, 47, 77, 15, 152, 240, 49] => Some("open_position".to_string()),
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
            [73, 69, 215, 228, 147, 128, 122, 185] => Some("sweep_deposit_queue".to_string()),
            [255, 36, 10, 1, 116, 246, 211, 64] => Some("deposit_aqua_farm".to_string()),
            [202, 175, 223, 211, 187, 171, 247, 82] => Some("deposit_double_dip".to_string()),
            [140, 55, 193, 2, 179, 135, 91, 155] => Some("deposit_double_dip_without_increase".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [136, 45, 191, 123, 212, 101, 215, 6] => Some("swap_tokens_orca_stats".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [135, 255, 2, 4, 95, 3, 19, 2] => Some("add_liquidity_stats".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [79, 168, 135, 119, 200, 42, 163, 68] => Some("deposit_vault".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [205, 75, 238, 108, 83, 210, 131, 252] => Some("deposit_orca_vault".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [1, 52, 121, 27, 157, 56, 250, 156] => {
                Some("deposit_orca_vault_without_shares".to_string())
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            [26, 84, 236, 102, 200, 190, 229, 121] => Some("deposit_orca_vault_dd".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [153, 167, 126, 15, 14, 127, 73, 33] => {
                Some("withdraw_raydium_vault_close".to_string())
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            [84, 194, 96, 63, 46, 145, 20, 150] => Some("withdraw_orca_vault_close".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [102, 165, 159, 226, 3, 168, 78, 178] => {
                Some("withdraw_orca_vault_dd_close".to_string())
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            [218, 23, 175, 249, 48, 64, 4, 236] => {
                Some("withdraw_orca_vault_without_shares".to_string())
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            [74, 48, 98, 96, 147, 29, 187, 203] => Some("remove_liquidity_new".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [39, 7, 137, 95, 44, 53, 38, 187] => Some("swap_tokens_serum".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [87, 188, 123, 29, 222, 194, 194, 153] => {
                Some("swap_tokens_to_repay_raydium".to_string())
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            [227, 103, 91, 198, 105, 18, 81, 247] => Some("swap_tokens_to_repay_orca".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [171, 61, 42, 106, 90, 144, 181, 10] => Some("repay_obligation_liquidity".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [53, 248, 147, 165, 236, 174, 30, 116] => Some("swap_tokens_raydium_stats".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [148, 156, 212, 239, 32, 220, 233, 152] => {
                Some("repay_obligation_liquidity_stats".to_string())
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            [81, 228, 219, 227, 27, 46, 245, 88] => Some("add_liquidity".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [151, 242, 97, 153, 158, 8, 189, 1] => {
                Some("repay_obligation_liquidity_external".to_string())
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            [210, 1, 24, 203, 253, 207, 100, 198] => Some("end_obligation_liquidation".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [101, 243, 117, 156, 44, 149, 16, 192] => Some("ray_liquidation_swap".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [26, 224, 58, 244, 99, 94, 139, 227] => Some("repay_liquidation_debt".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [224, 135, 7, 96, 141, 41, 181, 96] => {
                Some("spl_liquidation_swap_experimental".to_string())
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            [200, 61, 187, 32, 85, 205, 41, 237] => {
                Some("remove_liquidity_for_liquidation_improved".to_string())
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            [163, 29, 6, 223, 115, 172, 125, 232] => Some("pull_lp_for_liquidation".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [166, 98, 207, 5, 182, 104, 8, 112] => {
                Some("pull_lp_for_liquidation_spl_token_swap".to_string())
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            [239, 62, 39, 36, 36, 42, 147, 155] => {
                Some("pull_lp_for_liquidation_orca_without_shares".to_string())
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            [182, 155, 154, 61, 226, 108, 73, 91] => {
                Some("pull_lp_for_liquidation_double_dip".to_string())
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            [221, 137, 254, 158, 71, 124, 4, 35] => {
                Some("start_user_obligation_liquidation".to_string())
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            [197, 45, 4, 78, 201, 65, 227, 90] => Some("initialize".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [143, 208, 101, 111, 242, 172, 137, 76] => Some("set_product_account".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [138, 189, 124, 106, 120, 125, 125, 63] => Some("set_price".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [84, 175, 248, 228, 211, 176, 109, 166] => {
                Some("start_user_obligation_liquidation_new".to_string())
            }
            #[cfg(feature = "tulipv1-leverage-farm")]
            [39, 18, 162, 206, 228, 123, 13, 76] => Some("add_liq_issue_shares".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [198, 49, 192, 28, 9, 135, 3, 251] => Some("swap_to_repay_orca".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [30, 38, 34, 46, 211, 41, 152, 197] => Some("swap_to_repay_raydium".to_string()),
            #[cfg(feature = "tulipv1-leverage-farm")]
            [81, 238, 216, 99, 154, 78, 116, 173] => Some("empty_ix".to_string()),
            #[cfg(feature = "tulipv2")]
            [110, 72, 179, 47, 131, 109, 115, 103] => Some("issue_shares".to_string()),
            #[cfg(feature = "tulipv2")]
            [55, 114, 97, 238, 33, 173, 193, 225] => {
                Some("register_deposit_tracking_account".to_string())
            }
            #[cfg(feature = "tulipv2")]
            [3, 232, 22, 105, 242, 88, 178, 172] => Some("withdraw_deposit_tracking".to_string()),
            #[cfg(feature = "tulipv2")]
            [94, 147, 111, 141, 204, 247, 197, 86] => {
                Some("withdraw_multi_deposit_optimizer_vault".to_string())
            }
            #[cfg(feature = "tulipv2")]
            [249, 106, 178, 46, 61, 8, 85, 170] => Some("withdraw_sunny_vault".to_string()),
            #[cfg(feature = "tulipv2")]
            [254, 94, 225, 136, 118, 24, 139, 119] => Some("withdraw_raydium_vault".to_string()),
            #[cfg(feature = "tulipv2")]
            [29, 99, 95, 24, 128, 184, 10, 103] => Some("withdraw_atrix_vault".to_string()),
            #[cfg(feature = "tulipv2")]
            [227, 140, 117, 125, 113, 221, 30, 204] => {
                Some("withdraw_orca_vault_remove_liq".to_string())
            }
            #[cfg(feature = "tulipv2")]
            [166, 181, 35, 158, 120, 144, 104, 145] => {
                Some("withdraw_orca_vault_dd_stage_one".to_string())
            }
            #[cfg(feature = "tulipv2")]
            [210, 104, 7, 30, 239, 1, 235, 236] => {
                Some("withdraw_orca_vault_dd_stage_two".to_string())
            }
            #[cfg(feature = "tulipv2")]
            [177, 28, 79, 244, 220, 22, 29, 118] => Some("withdraw_orca_vault".to_string()),
            #[cfg(feature = "tulipv2")]
            [179, 206, 29, 224, 184, 84, 46, 227] => Some("withdraw_quarry_vault".to_string()),
            #[cfg(feature = "tulipv2")]
            [218, 21, 28, 71, 87, 52, 191, 88] => Some("orca_add_liq_issue_shares".to_string()),
            #[cfg(feature = "tulipv2")]
            [41, 244, 71, 189, 30, 118, 159, 171] => Some("permissioned_issue_shares".to_string()),
            #[cfg(feature = "tulipv2")]
            [163, 100, 239, 127, 131, 120, 124, 51] => {
                Some("rebase_lending_optimizer_vault".to_string())
            }
            #[cfg(feature = "tulipv2")]
            [59, 209, 5, 234, 109, 4, 43, 143] => {
                Some("rebase_multi_deposit_optimizer_vault".to_string())
            }
            _ => None,
        }
    }

    #[cfg(feature = "hex")]
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
    /// looks up the corresponding instruction sighash for the given instruction name
    /// using the anchor v6 for backwards compatability
    #[cfg(feature = "reverse-get")]
    pub fn get_deprecated(&self, val: &str) -> Option<[u8; 8]> {
        match val {
            "sweep_deposit_queue" => Some([246, 255, 134, 199, 150, 127, 51, 93]),
            "deposit_farm" => Some([255, 0, 109, 161, 120, 219, 45, 224]),
            "add_liquidity_stats" => Some([135, 255, 2, 4, 95, 3, 19, 2]),
            "deposit_orca_vault" => Some([205, 75, 238, 108, 83, 210, 131, 252]),
            "swap_tokens_orca_stats" => Some([136, 45, 191, 123, 212, 101, 215, 6]),
            "swap_tokens_raydium_stats" => Some([53, 248, 147, 165, 236, 174, 30, 116]),
            "deposit_vault" => Some([79, 168, 135, 119, 200, 42, 163, 68]),
            "deposit_orca_vault_without_shares" => Some([1, 52, 121, 27, 157, 56, 250, 156]),
            "deposit_orca_vault_dd" => Some([26, 84, 236, 102, 200, 190, 229, 121]),
            "orca_add_liquidity_queue" => Some([60, 51, 111, 125, 197, 208, 22, 248]),
            "withdraw_raydium_vault_close" => Some([153, 167, 126, 15, 14, 127, 73, 33]),
            "withdraw_orca_vault_without_shares" => Some([218, 23, 175, 249, 48, 64, 4, 236]),
            "withdraw_orca_vault_close" => Some([84, 194, 96, 63, 46, 145, 20, 150]),
            "withdraw_orca_vault" => Some([109, 97, 192, 91, 41, 89, 143, 196]),
            "withdraw_orca_vault_dd_close" => Some([102, 165, 159, 226, 3, 168, 78, 178]),
            "top_up_position_stats" => Some([205, 41, 21, 76, 102, 52, 30, 20]),
            "deposit_borrow_dual" => Some([170, 199, 78, 138, 119, 118, 96, 246]),
            "create_user_farm_obligation" => Some([193, 96, 101, 180, 136, 210, 148, 120]),
            "close_position_info_account" => Some([44, 205, 56, 239, 135, 45, 94, 116]),
            "create_user_farm" => Some([229, 49, 30, 92, 43, 69, 49, 220]),
            "set_price_extra" => Some([209, 65, 58, 32, 52, 135, 176, 148]),
            "withdraw_vault" => Some([89, 194, 163, 143, 0, 167, 190, 12]),
            _ => None,
        }
    }
    /// returns the corresponding instruction name for the given instruction sighash
    /// using the anchor v6 and below hashing method
    #[cfg(feature = "reverse-get")]
    pub fn reverse_get_deprecated(&self, val: [u8; 8]) -> Option<String> {
        match val {
            [246, 255, 134, 199, 150, 127, 51, 93] => Some("sweep_deposit_queue".to_string()),
            [255, 0, 109, 161, 120, 219, 45, 224] => Some("deposit_farm".to_string()),
            [135, 255, 2, 4, 95, 3, 19, 2] => Some("add_liquidity_stats".to_string()),
            [205, 75, 238, 108, 83, 210, 131, 252] => Some("deposit_orca_vault".to_string()),
            [136, 45, 191, 123, 212, 101, 215, 6] => Some("swap_tokens_orca_stats".to_string()),
            [53, 248, 147, 165, 236, 174, 30, 116] => Some("swap_tokens_raydium_stats".to_string()),
            [79, 168, 135, 119, 200, 42, 163, 68] => Some("deposit_vault".to_string()),
            [1, 52, 121, 27, 157, 56, 250, 156] => {
                Some("deposit_orca_vault_without_shares".to_string())
            }
            [26, 84, 236, 102, 200, 190, 229, 121] => Some("deposit_orca_vault_dd".to_string()),
            [60, 51, 111, 125, 197, 208, 22, 248] => Some("orca_add_liquidity_queue".to_string()),
            [153, 167, 126, 15, 14, 127, 73, 33] => {
                Some("withdraw_raydium_vault_close".to_string())
            }
            [218, 23, 175, 249, 48, 64, 4, 236] => {
                Some("withdraw_orca_vault_without_shares".to_string())
            }
            [84, 194, 96, 63, 46, 145, 20, 150] => Some("withdraw_orca_vault_close".to_string()),
            [109, 97, 192, 91, 41, 89, 143, 196] => Some("withdraw_orca_vault".to_string()),
            [102, 165, 159, 226, 3, 168, 78, 178] => {
                Some("withdraw_orca_vault_dd_close".to_string())
            }
            [205, 41, 21, 76, 102, 52, 30, 20] => Some("top_up_position_stats".to_string()),
            [170, 199, 78, 138, 119, 118, 96, 246] => Some("deposit_borrow_dual".to_string()),
            [193, 96, 101, 180, 136, 210, 148, 120] => {
                Some("create_user_farm_obligation".to_string())
            }
            [44, 205, 56, 239, 135, 45, 94, 116] => Some("close_position_info_account".to_string()),
            [229, 49, 30, 92, 43, 69, 49, 220] => Some("create_user_farm".to_string()),
            [209, 65, 58, 32, 52, 135, 176, 148] => Some("set_price_extra".to_string()),
            [89, 194, 163, 143, 0, 167, 190, 12] => Some("withdraw_vault".to_string()),
            _ => None,
        }
    }
}

#[cfg(feature = "hex")]
/// pares anchor instruction data for the first 8 bytes
/// which is the instruction sighash
pub fn extract_sighash_from_ix_data(data: &str) -> Option<[u8; 8]> {
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
    fn test_sighash_calculation() {
        {
            let mut context = Context::new(&SHA256);
            context.update(b"global:swap_tokens_raydium_stats");
            let digest = context.finish();
            println!("sighash {:?}", &digest.as_ref()[0..8]);
        }
    }
    #[test]
    fn test_global_sighashdb() {
        assert_eq!(
            GlobalSighashDB.get("deposit").unwrap(),
            [242, 35, 198, 137, 82, 225, 242, 182]
        );
        assert_eq!(
            GlobalSighashDB.get("create_staker").unwrap(),
            [14, 28, 165, 74, 243, 144, 108, 177]
        );
        assert_eq!(
            GlobalSighashDB.get("stake").unwrap(),
            [206, 176, 202, 18, 200, 209, 179, 108]
        );
        assert_eq!(
            GlobalSighashDB.get("stake_dual_crop").unwrap(),
            [241, 42, 177, 56, 14, 203, 117, 253]
        );
        assert_eq!(
            GlobalSighashDB.get("unstake").unwrap(),
            [90, 95, 107, 42, 205, 124, 50, 225]
        );
        assert_eq!(
            GlobalSighashDB.get("unstake_dual_crop").unwrap(),
            [125, 31, 2, 239, 223, 165, 240, 249]
        );
        assert_eq!(
            GlobalSighashDB.get("claim").unwrap(),
            [62, 198, 214, 193, 213, 159, 108, 210]
        );
        assert_eq!(
            GlobalSighashDB.get("claim_dual_crop").unwrap(),
            [128, 32, 146, 208, 138, 252, 110, 71]
        );
        assert_eq!(
            GlobalSighashDB.get("withdraw").unwrap(),
            [183, 18, 70, 156, 148, 109, 161, 34]
        );
        assert_eq!(
            GlobalSighashDB.get("create_harvester").unwrap(),
            [196, 93, 167, 138, 130, 242, 71, 148]
        );
        assert_eq!(
            GlobalSighashDB.get("place_ix").unwrap(),
            [173, 80, 54, 189, 140, 205, 196, 200]
        );
        assert_eq!(
            GlobalSighashDB.get("ix1").unwrap(),
            [142, 30, 183, 99, 253, 47, 35, 166]
        );
        assert_eq!(
            GlobalSighashDB.get("ix1_update_market").unwrap(),
            [29, 97, 43, 12, 156, 113, 217, 213]
        );
        assert_eq!(
            GlobalSighashDB.get("transfer_pool_lp").unwrap(),
            [126, 31, 82, 241, 144, 139, 110, 104]
        );
        assert_eq!(
            GlobalSighashDB.get("create_pool_oo").unwrap(),
            [182, 241, 212, 239, 26, 140, 19, 228]
        );
        assert_eq!(
            GlobalSighashDB.get("init_pool_oo").unwrap(),
            [49, 220, 157, 161, 132, 78, 15, 177]
        );
        assert_eq!(
            GlobalSighashDB.get("update_pool_orders").unwrap(),
            [135, 179, 178, 244, 103, 135, 218, 51]
        );
        assert_eq!(
            GlobalSighashDB.get("swap_tokens_orca_stats").unwrap(),
            [50, 33, 224, 173, 180, 200, 8, 129]
        );
        assert_eq!(
            GlobalSighashDB.get("add_liquidity_stats").unwrap(),
            [142, 249, 126, 143, 201, 62, 247, 95]
        );
        assert_eq!(
            GlobalSighashDB.get("deposit_vault").unwrap(),
            [126, 224, 21, 255, 228, 53, 117, 33]
        );
        assert_eq!(
            GlobalSighashDB.get("deposit_orca_vault").unwrap(),
            [164, 167, 93, 112, 242, 226, 239, 2]
        );
        assert_eq!(
            GlobalSighashDB
                .get("deposit_orca_vault_without_shares")
                .unwrap(),
            [252, 44, 185, 216, 172, 168, 113, 61]
        );
        assert_eq!(
            GlobalSighashDB.get("deposit_orca_vault_dd").unwrap(),
            [9, 222, 240, 221, 55, 125, 98, 69]
        );
        assert_eq!(
            GlobalSighashDB.get("withdraw_raydium_vault_close").unwrap(),
            [105, 95, 7, 156, 159, 74, 155, 68]
        );
        assert_eq!(
            GlobalSighashDB.get("withdraw_orca_vault_close").unwrap(),
            [64, 197, 169, 127, 139, 200, 224, 213]
        );
        assert_eq!(
            GlobalSighashDB.get("withdraw_orca_vault_dd_close").unwrap(),
            [204, 58, 193, 51, 153, 33, 192, 9]
        );
        assert_eq!(
            GlobalSighashDB
                .get("withdraw_orca_vault_without_shares")
                .unwrap(),
            [157, 249, 127, 73, 71, 78, 66, 252]
        );
        assert_eq!(
            GlobalSighashDB.get("remove_liquidity_new").unwrap(),
            [103, 162, 200, 156, 168, 32, 1, 175]
        );
        assert_eq!(
            GlobalSighashDB.get("swap_tokens_serum").unwrap(),
            [199, 81, 30, 13, 61, 153, 180, 175]
        );
        assert_eq!(
            GlobalSighashDB.get("swap_tokens_to_repay_raydium").unwrap(),
            [74, 177, 165, 198, 37, 93, 109, 72]
        );
        assert_eq!(
            GlobalSighashDB.get("swap_tokens_to_repay_orca").unwrap(),
            [122, 48, 109, 159, 227, 205, 239, 122]
        );
        assert_eq!(
            GlobalSighashDB.get("repay_obligation_liquidity").unwrap(),
            [145, 178, 13, 225, 76, 240, 147, 72]
        );
        assert_eq!(
            GlobalSighashDB.get("swap_tokens_raydium_stats").unwrap(),
            [234, 194, 146, 81, 192, 251, 43, 170]
        );
        assert_eq!(
            GlobalSighashDB
                .get("repay_obligation_liquidity_stats")
                .unwrap(),
            [222, 121, 26, 85, 108, 159, 243, 206]
        );
        assert_eq!(
            GlobalSighashDB.get("add_liquidity").unwrap(),
            [181, 157, 89, 67, 143, 182, 52, 72]
        );
    }
}

use anchor_lang::prelude::*;
use solana_sdk::pubkey;

pub const HEIST_SEED: &[u8] = b"heist";

pub const HEIST_COLLECTIONS: &[Pubkey; 1] =
    &[pubkey!("FQviQRFwrDCfUD1Ggwtw2uFxkBkW5fNR955YVGcYz7Ps")];

#[account]
pub struct UserLock {
    pub authority: Pubkey,
    pub locked_nfts: Vec<LockedNft>,
}

impl UserLock {
    pub fn calculate_space(raw_account: AccountInfo) -> usize {
        if raw_account.data_is_empty() {
            return 8 + 32 + 4;
        }

        raw_account.data_len()
    }

    pub const LEN: usize = 8 + 32 + 4;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub struct LockedNft {
    pub mint: Pubkey,
    pub locked_at: i64,
}

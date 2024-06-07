use anchor_lang::prelude::*;
use solana_sdk::pubkey;

pub const HEIST_SEED: &[u8] = b"heist";
pub const POINTS_SEED: &[u8] = b"points";

pub const HEIST_COLLECTIONS: &[Pubkey; 2] = &[
    pubkey!("6d9pvGuM6iG9GVuxRzSVHEQCdy44arm6oyqu6aUzrzLo"),
    pubkey!("9P47xMr4Z9jETNt8okSbr4VLmiUfaGWgE2vy3ECpeVN8"),
];

pub struct BurnData {
    pub collection_key: Pubkey,
    pub burn_collection: BurnCollection,
}

pub const BURN_DATA: &[BurnData; 2] = &[
    BurnData {
        burn_collection: BurnCollection::Remnants,
        collection_key: pubkey!("Dp4ZsLLkiZnGnNdPcFesvhtFzAEnxXgVe9vs2vWBjCjn"),
    },
    BurnData {
        burn_collection: BurnCollection::DefiPirates,
        collection_key: pubkey!("DRG3YRmurqpWQ1jEjK8DiWMuqPX9yL32LXLbuRdoiQwt"),
    },
];

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

#[account]
#[derive(InitSpace)]
pub struct UserPoints {
    pub total_points: u32,
    pub total_pirates_burnt: u32,
    pub total_remnants_burnt: u32,
    pub last_burnt_at: i64,
}

pub enum BurnCollection {
    Remnants,
    DefiPirates,
}

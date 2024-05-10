use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;
mod error;
pub mod state;
mod utils;

declare_id!("heis4XjeKqst1xSu36qy5u6CTTohXz6MVfvDCaKbn3S");

#[program]
pub mod heist_contract {
    use super::*;

    pub fn lock_nft(ctx: Context<LockUnlockNft>) -> Result<()> {
        instructions::lock_nft::lock_nft(ctx)
    }

    pub fn unlock_nft(ctx: Context<LockUnlockNft>) -> Result<()> {
        instructions::unlock_nft::unlock_nft(ctx)
    }

    pub fn burn_token(ctx: Context<BurnToken>) -> Result<()> {
        instructions::burn_token::burn_token(ctx)
    }
}

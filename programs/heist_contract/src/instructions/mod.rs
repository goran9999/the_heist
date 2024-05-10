use anchor_lang::prelude::*;

pub mod lock_nft;
use anchor_spl::token::Mint;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, TokenAccount},
};

use mpl_token_metadata::ID as MPX_ID;

pub mod burn_token;
pub use burn_token::*;
pub mod unlock_nft;

use crate::state::{UserLock, HEIST_SEED};

#[derive(Accounts)]
pub struct LockUnlockNft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,
    #[account(mut,seeds=[b"metadata",MPX_ID.as_ref(),nft_mint.key().as_ref()],seeds::program=MPX_ID,bump)]
    ///CHECK
    pub nft_metadata: UncheckedAccount<'info>,
    #[account(mut,seeds=[b"metadata",MPX_ID.as_ref(),nft_mint.key().as_ref(),b"edition"],seeds::program=MPX_ID,bump)]
    ///CHECK
    pub nft_edition: UncheckedAccount<'info>,
    #[account(init_if_needed,seeds=[HEIST_SEED,payer.key().as_ref()],bump,
    payer=payer,space=UserLock::calculate_space(user_lock.to_account_info()))]
    pub user_lock: Account<'info, UserLock>,
    #[account(mut,token::authority=payer,token::mint=nft_mint)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    ///CHECK
    pub token_record: UncheckedAccount<'info>,
    #[account()]
    ///CHECK
    pub authorization_rules: Option<AccountInfo<'info>>,
    #[account(executable)]
    ///CHECK
    pub authorization_rules_program: UncheckedAccount<'info>,
    #[account(address=mpl_token_metadata::ID)]
    ///CHECK
    pub token_metadata: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account()]
    ///CHECK
    pub sysvar_instructions: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

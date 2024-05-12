use crate::{
    error::HeistError,
    state::{UserLock, HEIST_SEED},
    utils::authority_guard,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use mpl_token_metadata::{
    instructions::{BurnCpi, BurnCpiAccounts, BurnInstructionArgs, UnlockCpi, UnlockCpiAccounts},
    types::{BurnArgs, UnlockArgs},
    ID as MPL_TOKEN_METADATA_ID,
};

#[derive(Accounts)]
pub struct BurnToken<'info> {
    #[account(constraint=authority_guard(&authority))]
    pub authority: Signer<'info>,
    #[account(mut,seeds=[HEIST_SEED,user.key().as_ref()],bump)]
    pub user_lock: Account<'info, UserLock>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    ///CHECK
    pub user: UncheckedAccount<'info>,
    #[account(mut,seeds=[b"metadata",MPL_TOKEN_METADATA_ID.as_ref(),mint.key().as_ref()]
    ,bump,seeds::program=MPL_TOKEN_METADATA_ID)]
    ///CHECK
    pub nft_metadata: UncheckedAccount<'info>,
    #[account(mut,seeds=[b"metadata",MPL_TOKEN_METADATA_ID.as_ref(),mint.key().as_ref(),b"edition"]
    ,bump,seeds::program=MPL_TOKEN_METADATA_ID)]
    ///CHECK
    pub edition: UncheckedAccount<'info>,
    #[account()]
    ///CHECK
    pub collection_metadata: UncheckedAccount<'info>,
    #[account()]
    ///CHECK
    pub token_record: UncheckedAccount<'info>,
    #[account()]
    ///CHECK
    pub authorization_rules: Option<AccountInfo<'info>>,
    #[account(token::mint=mint.key())]
    pub token_account: Account<'info, TokenAccount>,
    #[account(address=MPL_TOKEN_METADATA_ID)]
    ///CHECK
    pub mpl_token_metadata: UncheckedAccount<'info>,
    ///CHECK
    pub authorization_rules_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    ///CHECK
    pub sysvar_instructions: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn burn_nft(ctx: Context<BurnToken>) -> Result<()> {
    let user_lock = &mut ctx.accounts.user_lock;

    require!(
        user_lock
            .locked_nfts
            .iter()
            .any(|n| n.mint == ctx.accounts.mint.key()),
        HeistError::UserDidNotStake
    );

    user_lock
        .locked_nfts
        .retain(|nft| nft.mint != ctx.accounts.mint.key());

    UnlockCpi::new(
        &ctx.accounts.mpl_token_metadata,
        UnlockCpiAccounts {
            authority: &ctx.accounts.user_lock.to_account_info(),
            authorization_rules: ctx.accounts.authorization_rules.as_ref(),
            authorization_rules_program: Some(&ctx.accounts.authorization_rules_program),
            metadata: &ctx.accounts.nft_metadata,
            mint: &ctx.accounts.mint.to_account_info(),
            payer: &ctx.accounts.authority.to_account_info(),
            edition: Some(&ctx.accounts.edition.to_account_info()),
            spl_token_program: Some(&ctx.accounts.token_program.to_account_info()),
            system_program: &ctx.accounts.system_program.to_account_info(),
            token: &ctx.accounts.token_account.to_account_info(),
            token_record: Some(&ctx.accounts.token_record.to_account_info()),
            sysvar_instructions: &ctx.accounts.sysvar_instructions,
            token_owner: Some(&ctx.accounts.authority.to_account_info()),
        },
        mpl_token_metadata::instructions::UnlockInstructionArgs {
            unlock_args: UnlockArgs::V1 {
                authorization_data: None,
            },
        },
    )
    .invoke_signed(&[&[
        HEIST_SEED,
        ctx.accounts.user.key().as_ref(),
        &[ctx.bumps.user_lock],
    ]])?;

    BurnCpi::new(
        &ctx.accounts.mpl_token_metadata,
        BurnCpiAccounts {
            authority: &ctx.accounts.user_lock.to_account_info(),
            collection_metadata: Some(&ctx.accounts.collection_metadata.to_account_info()),
            master_edition: Some(&ctx.accounts.edition.to_account_info()),
            metadata: &ctx.accounts.nft_metadata.to_account_info(),
            mint: &ctx.accounts.mint.to_account_info(),
            edition: Some(&ctx.accounts.edition.to_account_info()),
            spl_token_program: &ctx.accounts.token_program,
            token: &ctx.accounts.token_account.to_account_info(),
            system_program: &ctx.accounts.system_program.to_account_info(),
            sysvar_instructions: &ctx.accounts.sysvar_instructions.to_account_info(),
            token_record: Some(&ctx.accounts.token_record.to_account_info()),
            edition_marker: None,
            master_edition_mint: None,
            master_edition_token: None,
        },
        BurnInstructionArgs {
            burn_args: BurnArgs::V1 { amount: 1 },
        },
    )
    .invoke_signed(&[&[
        HEIST_SEED,
        ctx.accounts.user.key().as_ref(),
        &[ctx.bumps.user_lock],
    ]])?;

    Ok(())
}

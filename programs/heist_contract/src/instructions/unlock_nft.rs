use anchor_lang::prelude::*;
use mpl_token_metadata::{
    instructions::{
        RevokeLockedTransferV1Cpi, RevokeLockedTransferV1CpiAccounts, UnlockCpi, UnlockCpiAccounts,
    },
    types::UnlockArgs,
};

use crate::{error::HeistError, state::HEIST_SEED, utils::check_metadata};

use super::LockUnlockNft;

pub fn unlock_nft(ctx: Context<LockUnlockNft>) -> Result<()> {
    let user_lock = &mut ctx.accounts.user_lock;

    check_metadata(ctx.accounts.nft_metadata.to_account_info())?;

    require!(
        user_lock
            .locked_nfts
            .iter()
            .any(|n| n.mint == ctx.accounts.nft_mint.key()),
        HeistError::UserDidNotStake
    );

    user_lock
        .locked_nfts
        .retain(|nft| nft.mint != ctx.accounts.nft_mint.key());

    UnlockCpi::new(
        &ctx.accounts.token_metadata,
        UnlockCpiAccounts {
            authority: &user_lock.to_account_info(),
            authorization_rules: ctx.accounts.authorization_rules.as_ref(),
            authorization_rules_program: Some(&ctx.accounts.authorization_rules_program),
            metadata: &ctx.accounts.nft_metadata,
            mint: &ctx.accounts.nft_mint.to_account_info(),
            payer: &ctx.accounts.payer.to_account_info(),
            edition: Some(&ctx.accounts.nft_edition.to_account_info()),
            spl_token_program: Some(&ctx.accounts.token_program.to_account_info()),
            system_program: &ctx.accounts.system_program.to_account_info(),
            token: &ctx.accounts.token_account.to_account_info(),
            token_record: Some(&ctx.accounts.token_record.to_account_info()),
            sysvar_instructions: &ctx.accounts.sysvar_instructions,
            token_owner: Some(&ctx.accounts.payer.to_account_info()),
        },
        mpl_token_metadata::instructions::UnlockInstructionArgs {
            unlock_args: UnlockArgs::V1 {
                authorization_data: None,
            },
        },
    )
    .invoke_signed(&[&[
        HEIST_SEED,
        ctx.accounts.payer.key().as_ref(),
        &[ctx.bumps.user_lock],
    ]])?;

    RevokeLockedTransferV1Cpi::new(
        &ctx.accounts.token_metadata,
        RevokeLockedTransferV1CpiAccounts {
            authority: &ctx.accounts.payer.to_account_info(),
            authorization_rules: ctx.accounts.authorization_rules.as_ref(),
            authorization_rules_program: Some(&ctx.accounts.authorization_rules_program),
            metadata: &ctx.accounts.nft_metadata,
            mint: &ctx.accounts.nft_mint.to_account_info(),
            payer: &ctx.accounts.payer.to_account_info(),
            master_edition: Some(&ctx.accounts.nft_edition.to_account_info()),
            spl_token_program: Some(&ctx.accounts.token_program.to_account_info()),
            system_program: &ctx.accounts.system_program.to_account_info(),
            token: &ctx.accounts.token_account.to_account_info(),
            token_record: Some(&ctx.accounts.token_record.to_account_info()),
            sysvar_instructions: &ctx.accounts.sysvar_instructions,
            delegate: &user_lock.to_account_info(),
            delegate_record: None,
        },
    )
    .invoke()?;

    Ok(())
}

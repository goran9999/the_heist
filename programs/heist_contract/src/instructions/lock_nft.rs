use std::ops::{Add, Mul, Sub};

use anchor_lang::prelude::*;

use mpl_token_metadata::{
    instructions::{
        DelegateCpi, DelegateCpiAccounts, DelegateInstructionArgs, LockCpi, LockCpiAccounts,
        LockInstructionArgs,
    },
    types::DelegateArgs,
};

use crate::{
    state::{LockedNft, UserLock, HEIST_SEED},
    utils::check_metadata,
};

use super::LockUnlockNft;

pub fn lock_nft(ctx: Context<LockUnlockNft>) -> Result<()> {
    let user_lock = &mut ctx.accounts.user_lock;

    if user_lock.authority == Pubkey::default() {
        user_lock.authority = ctx.accounts.payer.key();
        user_lock.locked_nfts = vec![];
    }

    let account_len = user_lock.to_account_info().data_len();

    let new_space = UserLock::LEN.add((user_lock.locked_nfts.len() + 1).mul(LockedNft::INIT_SPACE));

    check_metadata(ctx.accounts.nft_metadata.to_account_info())?;

    //we check if we should realloc account on new lock because
    //when someone unlocks nft and we remove nft from array,we can't realloc to lower space
    if new_space > account_len {
        let additional_bytes = new_space.sub(account_len);

        let additional_rent = Rent::default().minimum_balance(additional_bytes);

        anchor_lang::system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: user_lock.to_account_info(),
                },
            ),
            additional_rent,
        )?;

        user_lock
            .to_account_info()
            .realloc(account_len.add(additional_bytes), false)?;
    }

    user_lock.locked_nfts.push(LockedNft {
        mint: ctx.accounts.nft_mint.key(),
        locked_at: Clock::get().unwrap().unix_timestamp,
    });

    DelegateCpi::new(
        &ctx.accounts.token_metadata,
        DelegateCpiAccounts {
            authority: &ctx.accounts.payer.to_account_info(),
            authorization_rules: ctx.accounts.authorization_rules.as_ref(),
            authorization_rules_program: Some(&ctx.accounts.authorization_rules_program),
            delegate: &user_lock.to_account_info(),
            master_edition: Some(&ctx.accounts.nft_edition),
            delegate_record: None,
            metadata: &ctx.accounts.nft_metadata,
            mint: &ctx.accounts.nft_mint.to_account_info(),
            payer: &ctx.accounts.payer.to_account_info(),
            spl_token_program: Some(&ctx.accounts.token_program.to_account_info()),
            system_program: &ctx.accounts.system_program.to_account_info(),
            token: Some(&ctx.accounts.token_account.to_account_info()),
            token_record: Some(&ctx.accounts.token_record.to_account_info()),
            sysvar_instructions: &ctx.accounts.sysvar_instructions,
        },
        DelegateInstructionArgs {
            delegate_args: DelegateArgs::LockedTransferV1 {
                amount: 1,
                locked_address: user_lock.key(),
                authorization_data: None,
            },
        },
    )
    .invoke()?;

    LockCpi::new(
        &ctx.accounts.token_metadata,
        LockCpiAccounts {
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
        LockInstructionArgs {
            lock_args: mpl_token_metadata::types::LockArgs::V1 {
                authorization_data: None,
            },
        },
    )
    .invoke_signed(&[&[
        HEIST_SEED,
        ctx.accounts.payer.key().as_ref(),
        &[ctx.bumps.user_lock],
    ]])?;

    Ok(())
}

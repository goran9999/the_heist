use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use mpl_token_metadata::{
    accounts::Metadata,
    instructions::{BurnV1Cpi, BurnV1CpiAccounts, BurnV1InstructionArgs},
    types::Collection,
    ID as MPL_TOKEN_METADATA,
};

use crate::{
    error::HeistError,
    state::{BurnCollection, UserPoints, BURN_DATA, POINTS_SEED},
};
#[derive(Accounts)]
pub struct BurnForPoints<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,
    #[account(mut,seeds=[b"metadata",MPL_TOKEN_METADATA.as_ref(),nft_mint.key().as_ref()],bump,seeds::program=MPL_TOKEN_METADATA)]
    ///CHECK
    pub nft_metadata: UncheckedAccount<'info>,
    #[account(init_if_needed,seeds=[POINTS_SEED,payer.key().as_ref()],bump,payer=payer,space=8+UserPoints::INIT_SPACE)]
    pub user_points: Account<'info, UserPoints>,
    #[account(mut)]
    ///CHECK
    pub nft_edition: UncheckedAccount<'info>,
    #[account(mut,token::mint=nft_mint,token::authority=payer)]
    pub token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(address=MPL_TOKEN_METADATA)]
    ///CHECK
    pub mpl_token_metadata: UncheckedAccount<'info>,
    ///CHECK
    pub sysvar_instructions: UncheckedAccount<'info>,
}

pub fn burn_for_points<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, BurnForPoints<'info>>,
) -> Result<()> {
    let user_points = &mut ctx.accounts.user_points;

    let decoded_metadata = Metadata::safe_deserialize(&ctx.accounts.nft_metadata.data.borrow())?;

    let collection = decoded_metadata.collection;

    msg!("Coll {:?}", collection);
    let found_collection = BURN_DATA
        .iter()
        .find(|c| {
            c.collection_key
                == collection
                    .clone()
                    .unwrap_or(Collection {
                        key: Pubkey::default(),
                        verified: false,
                    })
                    .key
                || c.collection_key
                    == decoded_metadata
                        .creators
                        .clone()
                        .unwrap()
                        .get(0)
                        .unwrap()
                        .address
        })
        .expect("Invalid collection");

    if collection.is_some() && !collection.unwrap().verified {
        return err!(HeistError::InvalidCollection);
    }

    let points = match found_collection.burn_collection {
        BurnCollection::DefiPirates => {
            msg!("Burning DeFi Pirate");
            user_points.total_pirates_burnt += 1;
            25u32
        }
        BurnCollection::Remnants => {
            msg!("Burning Remnant");

            user_points.total_remnants_burnt += 1;

            50u32
        }
    };

    let remaining_accounts = &mut ctx.remaining_accounts.iter();

    let collection_metadata = if let Some(collection_meta) = remaining_accounts.next() {
        Some(collection_meta)
    } else {
        None
    };

    user_points.last_burnt_at = Clock::get().unwrap().unix_timestamp;
    user_points.total_points = user_points.total_points.checked_add(points).unwrap();

    BurnV1Cpi::new(
        &ctx.accounts.mpl_token_metadata.to_account_info(),
        BurnV1CpiAccounts {
            authority: &ctx.accounts.payer,
            collection_metadata,
            edition: Some(&ctx.accounts.nft_edition),
            edition_marker: None,
            master_edition: None,
            master_edition_mint: None,
            master_edition_token: None,
            metadata: &ctx.accounts.nft_metadata,
            mint: &ctx.accounts.nft_mint.to_account_info(),
            spl_token_program: &ctx.accounts.token_program.to_account_info(),
            system_program: &ctx.accounts.system_program.to_account_info(),
            token: &ctx.accounts.token_account.to_account_info(),
            token_record: None,
            sysvar_instructions: &ctx.accounts.sysvar_instructions,
        },
        BurnV1InstructionArgs { amount: 1 },
    )
    .invoke()?;

    Ok(())
}

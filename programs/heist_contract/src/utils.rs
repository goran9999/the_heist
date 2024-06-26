use anchor_lang::prelude::*;
use mpl_token_metadata::accounts::Metadata;
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

use crate::{error::HeistError, state::HEIST_COLLECTIONS};

pub fn check_metadata(raw_metadata: AccountInfo) -> Result<Pubkey> {
    let metadata = Metadata::safe_deserialize(&raw_metadata.data.borrow())?;

    let collection = metadata.collection.expect("Invalid collection!");

    require!(
        HEIST_COLLECTIONS.iter().any(|c| *c == collection.key) && collection.verified,
        HeistError::InvalidCollection
    );

    Ok(collection.key)
}

pub const PLATFORM_AUTHORITY: &str = "6RaQPoxaR1dzAmdPnPaiALbr6D2BaZ2YfgVnvXWJCDau";

pub fn authority_guard(account: &AccountInfo) -> bool {
    account.key().to_string() == PLATFORM_AUTHORITY
}

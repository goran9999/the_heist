use anchor_lang::error_code;

#[error_code]
pub enum HeistError {
    #[msg("Invalid collection")]
    InvalidCollection,
    #[msg("Invalid NFT owner")]
    InvalidOwner,
}

use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum SchoolType {
    HighSchool,
    College,
    University,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct CreateNFTParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub const MAX_NAME_LENGTH: usize = 32;
pub const MAX_SYMBOL_LENGTH: usize = 10;
pub const RATE_LIMIT_PERIOD: i64 = 3600;
pub const RATE_LIMIT_COUNT: u64 = 5;

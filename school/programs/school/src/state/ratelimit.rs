use anchor_lang::prelude::*;
#[account]
pub struct RateLimit {
    pub last_mint_time: i64,
    pub mint_count: u64,
}

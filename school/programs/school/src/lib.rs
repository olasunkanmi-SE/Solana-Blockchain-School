use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;
use state::*;

declare_id!("EbksbaY1aGV2vxDtpmFU5zv52UxsefiY5GbmcvUTffap");

#[program]
pub mod school {

    use super::*;

    pub fn initialize_school(
        ctx: Context<InitializeSchool>,
        enrollment_fee: u64,
        name: String,
        school_type: String,
        fee_multiplier: u64,
    ) -> Result<()> {
        instructions::initialize_school(ctx, enrollment_fee, name, school_type, fee_multiplier)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

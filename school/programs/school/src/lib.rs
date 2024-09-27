use anchor_lang::prelude::*;

declare_id!("EbksbaY1aGV2vxDtpmFU5zv52UxsefiY5GbmcvUTffap");

#[program]
pub mod school {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

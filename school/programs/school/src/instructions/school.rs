use anchor_lang::prelude::*;

use crate::{School, SchoolTrait};

#[derive(Accounts)]
#[instruction(name: String)]

pub struct IntializeSchool<'info> {
    #[account(init, payer=authority, space= 8 + 32 + 8 + 4 + 4, seeds=[b"school".as_ref(), name.as_bytes().as_ref()], bump)]
    pub school: Account<'info, School>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_school(
    ctx: Context<IntializeSchool>,
    enrolment_fee: u64,
    name: String,
) -> Result<()> {
    let school = &mut ctx.accounts.school;
    school.set_authority(ctx.accounts.authority.key());
    school.set_enrolment_fee(enrolment_fee);
    school.set_name(name);
    Ok(())
}

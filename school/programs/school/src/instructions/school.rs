use anchor_lang::prelude::*;

use crate::{constants::SchoolType, School, SchoolTrait};

#[derive(Accounts)]

pub struct IntializeSchool<'info> {
    #[account(init, payer=authority, space= 8 + 32 + 8 + 4 + 4, seeds=[b"school".as_ref(), authority.key().as_ref()], bump)]
    pub school: Account<'info, School>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_school(
    ctx: Context<IntializeSchool>,
    enrolment_fee: u64,
    name: String,
    school_type: SchoolType,
    fee_multiplier: f32,
) -> Result<()> {
    let school = &mut ctx.accounts.school;
    school.set_authority(ctx.accounts.authority.key());
    school.set_enrollment_fee(enrolment_fee);
    school.set_school_type(school_type);
    school.set_fee_multiplier(fee_multiplier);
    school.set_name(name);
    school.set_bump(ctx.bumps.school);
    Ok(())
}

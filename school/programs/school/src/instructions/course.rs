use anchor_lang::prelude::*;

use crate::{Course, RateLimit, School, SchoolTrait};

#[derive(Accounts)]
#[instruction(name: String, school_bump: u8)]
pub struct InitializesCourse<'info> {
    #[account(init, payer=authority, space= 8 + 32 + 8 + 4, seeds=[b"course", name.as_bytes(), school.key().as_ref(), authority.key().as_ref()], bump)]
    pub course: Account<'info, Course>,
    #[account(mut)]
    authority: Signer<'info>,
    #[account(seeds=[b"school", authority.key().as_ref(), &school.get_school_type().as_bytes()], bump = school_bump)]
    pub school: Account<'info, School>,
    pub system_program: Program<'info, System>,
    #[account(
        mut, seeds= [b"rate_limit", authority.key().as_ref()], bump
    )]
    pub rate_limit: Account<'info, RateLimit>,
    pub rent: Sysvar<'info, Rent>,
}

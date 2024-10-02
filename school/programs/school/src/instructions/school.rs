use anchor_lang::prelude::*;

use crate::{constants::SchoolType, error::SchoolError, School, SchoolTrait};

#[derive(Accounts)]
#[instruction(school_type: String)]
pub struct InitializeSchool<'info> {
    #[account(init, payer=authority, space= 8 + 32 + 8 + 4 + 4, seeds=[b"school".as_ref(), authority.key().as_ref(), &school_type.as_bytes()], bump)]
    pub school: Account<'info, School>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_school(
    ctx: Context<InitializeSchool>,
    enrollment_fee: u64,
    name: String,
    school_type: String,
    fee_multiplier: u64,
) -> Result<()> {
    let school_type = match school_type.as_str() {
        "HighSchool" => SchoolType::HighSchool,
        "College" => SchoolType::College,
        "University" => SchoolType::University,
        _ => return Err(SchoolError::InvalidSchoolType.into()),
    };
    let school = &mut ctx.accounts.school;
    school.set_authority(ctx.accounts.authority.key());
    school.set_enrollment_fee(enrollment_fee);
    school.set_school_type(school_type);
    school.set_fee_multiplier(fee_multiplier)?;
    school.set_name(name);
    school.set_bump(ctx.bumps.school);
    Ok(())
}

use anchor_lang::prelude::*;

use crate::constants::SchoolType;

#[account]
pub struct School {
    name: String,
    authority: Pubkey,
    base_enrollment_fee: u64,
    class_count: u64,
    book_count: u64,
    student_count: u64,
    school_type: SchoolType,
    fee_multiplier: u64,
    bump: u8,
}

pub trait SchoolTrait {
    fn new(
        authority: Pubkey,
        base_enrollment_fee: u64,
        fee_multiplier: u64,
        name: String,
        school_type: SchoolType,
    ) -> Self;
    fn get_name(&self) -> &str;
    fn get_authority(&self) -> Pubkey;
    fn get_enrollment_fee(&self) -> u64;
    fn get_fee_multiplier(&self) -> u64;
    fn class_count(&self) -> u64;
    fn book_count(&self) -> u64;
    fn student_count(&self) -> u64;
    fn increment_class_count(&mut self) -> Result<()>;
    fn increment_book_count(&mut self) -> Result<()>;
    fn increment_student_count(&mut self) -> Result<()>;
    fn set_enrollment_fee(&mut self, new_fee: u64);
    fn set_fee_multiplier(&mut self, new_multiplier: u64) -> Result<()>;
    fn set_authority(&mut self, new_authority: Pubkey);
    fn set_name(&mut self, name: String);
    fn get_school_type(&self) -> SchoolType;
    fn set_school_type(&mut self, school_type: SchoolType);
    fn calculate_enrollment_fees(&self) -> Result<u64>;
    fn get_bump(&self) -> u8;
    fn set_bump(&mut self, bump: u8);
}

impl SchoolTrait for School {
    fn new(
        authority: Pubkey,
        base_enrollment_fee: u64,
        fee_multiplier: u64,
        name: String,
        school_type: SchoolType,
    ) -> Self {
        School {
            authority,
            base_enrollment_fee,
            fee_multiplier,
            name,
            school_type,
            class_count: 0,
            book_count: 0,
            student_count: 0,
            bump: 0,
        }
    }

    fn set_school_type(&mut self, school_type: SchoolType) {
        self.school_type = school_type
    }

    fn get_school_type(&self) -> SchoolType {
        self.school_type
    }

    fn get_authority(&self) -> Pubkey {
        self.authority
    }

    fn get_enrollment_fee(&self) -> u64 {
        self.base_enrollment_fee
    }

    fn get_fee_multiplier(&self) -> u64 {
        self.fee_multiplier
    }

    fn class_count(&self) -> u64 {
        self.class_count
    }

    fn book_count(&self) -> u64 {
        self.book_count
    }

    fn student_count(&self) -> u64 {
        self.student_count
    }

    fn increment_class_count(&mut self) -> Result<()> {
        self.class_count = self
            .class_count
            .checked_add(1)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        Ok(())
    }

    fn increment_book_count(&mut self) -> Result<()> {
        self.book_count = self
            .book_count
            .checked_add(1)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        Ok(())
    }

    fn increment_student_count(&mut self) -> Result<()> {
        self.student_count = self
            .student_count
            .checked_add(1)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        Ok(())
    }

    fn set_enrollment_fee(&mut self, new_fee: u64) {
        self.base_enrollment_fee = new_fee;
    }

    fn set_authority(&mut self, new_authority: Pubkey) {
        self.authority = new_authority
    }

    fn set_fee_multiplier(&mut self, new_multiplier: u64) -> Result<()> {
        if new_multiplier == 0 {
            return Err(ProgramError::InvalidArgument)?;
        }
        self.fee_multiplier = new_multiplier;
        Ok(())
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn calculate_enrollment_fees(&self) -> Result<u64> {
        let fee_in_lamports = self
            .base_enrollment_fee
            .checked_mul(1_000_000)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        let result = fee_in_lamports
            .checked_mul(self.fee_multiplier)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(result)
    }

    fn get_bump(&self) -> u8 {
        self.bump
    }
    fn set_bump(&mut self, bump: u8) {
        self.bump = bump
    }
}

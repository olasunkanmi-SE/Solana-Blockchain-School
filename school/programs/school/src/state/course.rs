use anchor_lang::prelude::*;

#[account]
pub struct Course {
    name: String,
    nft_mint: Pubkey,
    nft_token_account: Pubkey,
    capacity: u32,
    enrolled_students_count: u32,
    tutuion_fee: u64,
    school: Pubkey,
    authority: Pubkey,
    bump: u8,
}

pub trait CourseTrait {
    fn new(
        name: String,
        nft_mint: Pubkey,
        capacity: u32,
        enrolled_students_count: u32,
        tutuion_fee: u64,
        school: Pubkey,
        authority: Pubkey,
        nft_token_account: Pubkey,
        bump: u8,
    ) -> Self;

    fn get_name(&self) -> &str;
    fn get_nft_mint(&self) -> Pubkey;
    fn get_capacity(&self) -> u32;
    fn get_enrolled_students_count(&self) -> u32;
    fn get_tution_fee(&self) -> u64;
    fn get_school(&self) -> Pubkey;
    fn get_authority(&self) -> Pubkey;
    fn get_nft_token_account(&self) -> Pubkey;

    fn set_name(&mut self, name: String);
    fn set_nft_mint(&mut self, nft_mint: Pubkey);
    fn set_capacity(&mut self, capacity: u32);
    fn set_enrolled_students_count(&mut self, enrolled_students_count: u32);
    fn set_tution_fee(&mut self, tutuion_fee: u64);
    fn set_school(&mut self, school: Pubkey);
    fn set_authority(&mut self, authority: Pubkey);
    fn get_bump(&self) -> u8;
    fn set_bump(&mut self, bump: u8);
}

impl CourseTrait for Course {
    fn new(
        name: String,
        nft_mint: Pubkey,
        capacity: u32,
        enrolled_students_count: u32,
        tutuion_fee: u64,
        school: Pubkey,
        authority: Pubkey,
        nft_token_account: Pubkey,
        bump: u8,
    ) -> Self {
        Course {
            name,
            nft_mint,
            nft_token_account,
            capacity,
            enrolled_students_count,
            tutuion_fee,
            school,
            authority,
            bump,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_nft_mint(&self) -> Pubkey {
        self.nft_mint
    }

    fn get_authority(&self) -> Pubkey {
        self.authority
    }

    fn get_capacity(&self) -> u32 {
        self.capacity
    }
    fn get_enrolled_students_count(&self) -> u32 {
        self.enrolled_students_count
    }
    fn get_nft_token_account(&self) -> Pubkey {
        self.nft_token_account
    }
    fn get_school(&self) -> Pubkey {
        self.school
    }
    fn get_tution_fee(&self) -> u64 {
        self.tutuion_fee
    }
    fn get_bump(&self) -> u8 {
        self.bump
    }
    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn set_authority(&mut self, authority: Pubkey) {
        self.authority = authority
    }

    fn set_bump(&mut self, bump: u8) {
        self.bump = bump
    }

    fn set_capacity(&mut self, capacity: u32) {
        self.capacity = capacity
    }

    fn set_enrolled_students_count(&mut self, enrolled_students_count: u32) {
        self.enrolled_students_count = enrolled_students_count
    }

    fn set_nft_mint(&mut self, nft_mint: Pubkey) {
        self.nft_mint = nft_mint
    }

    fn set_school(&mut self, school: Pubkey) {
        self.school = school
    }

    fn set_tution_fee(&mut self, tutuion_fee: u64) {
        self.tutuion_fee = tutuion_fee
    }
}

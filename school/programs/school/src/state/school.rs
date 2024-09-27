use anchor_lang::prelude::*;

#[account]
pub struct School {
    name: String,
    authority: Pubkey,
    enrolment_fee: u64,
    class_count: u64,
    book_count: u64,
    student_count: u64,
}

pub trait SchoolTrait {
    fn new(authority: Pubkey, enrolment_fee: u64, name: String) -> Self;
    fn get_name(&self) -> &str;
    fn get_authority(&self) -> Pubkey;
    fn get_enrolment_fee(&self) -> u64;
    fn class_count(&self) -> u64;
    fn book_count(&self) -> u64;
    fn student_count(&self) -> u64;
    fn increment_class_count(&mut self);
    fn increment_book_count(&mut self);
    fn increment_student_count(&mut self);
    fn set_enrolment_fee(&mut self, new_fee: u64);
    fn set_authority(&mut self, new_authority: Pubkey);
    fn set_name(&mut self, name: String);
}

impl SchoolTrait for School {
    fn new(authority: Pubkey, enrolment_fee: u64, name: String) -> Self {
        School {
            authority,
            enrolment_fee,
            name,
            class_count: 0,
            book_count: 0,
            student_count: 0,
        }
    }

    fn get_authority(&self) -> Pubkey {
        self.authority
    }

    fn get_enrolment_fee(&self) -> u64 {
        self.enrolment_fee
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

    fn increment_class_count(&mut self) {
        self.class_count += 1
    }

    fn increment_book_count(&mut self) {
        self.book_count += 1
    }

    fn increment_student_count(&mut self) {
        self.student_count += 1
    }

    fn set_enrolment_fee(&mut self, new_fee: u64) {
        self.enrolment_fee = new_fee;
    }

    fn set_authority(&mut self, new_authority: Pubkey) {
        self.authority = new_authority
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name
    }
}

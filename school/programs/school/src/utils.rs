use anchor_lang::prelude::*;

use crate::constants::SchoolType;

pub trait SchoolTypeTrait {
    fn is_school_type(sch_type: &str) -> Result<bool>;
}

impl SchoolTypeTrait for SchoolType {
    fn is_school_type(sch_type: &str) -> Result<bool> {
        match sch_type.to_lowercase().as_str() {
            "highschool" => Ok(true),
            "college" => Ok(true),
            "university" => Ok(true),
            _ => Ok(false),
        }
    }
}

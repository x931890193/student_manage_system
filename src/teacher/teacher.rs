
use crate::curse::Curse;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Teacher {
    pub id: u8,
    pub name: String,
    pub age: u8,
}



impl Teacher {
    pub fn new(id: u8, name: String, age: u8) -> Teacher {
        Teacher {
            id,
            name,
            age,
        }
    }
}
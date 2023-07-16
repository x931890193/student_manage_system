use serde::{Deserialize, Serialize};
use crate::class::Class;
use crate::curse::Curse;

#[derive(Debug)]
pub struct Student {
    pub id: u8,
    pub name: String,
    pub age: u8,
    pub number: u8,
    pub create_time: String,
    pub modify_time: String,
}

impl Student {
    pub fn new(id: u8, name: String, age: u8, number: u8, create_time: String, modify_time: String) -> Student {
        Student {
            id,
            name,
            age,
            number,
            create_time,
            modify_time,
        }
    }
}
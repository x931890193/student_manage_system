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
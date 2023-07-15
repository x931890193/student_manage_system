
use crate::curse::Curse;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Teacher {
    pub id: u8,
    pub name: String,
    pub age: u8,
    pub curses: Vec<Curse>,
}
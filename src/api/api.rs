use crate::user::User;
use crate::user::Role;
use crate::storage::Storage;
use crate::teacher::Teacher;
use crate::student::Student;

#[derive(Debug, Clone)]
pub struct API;

impl API {
    pub fn new() -> API {
        API
    }
    pub fn register(&self) {
        unimplemented!("Register");
    }

    pub fn login(&self, username: String, password: String) -> User {
        unimplemented!("Login");
    }

    pub fn logout(&self) {
        unimplemented!("Logout");
    }
}
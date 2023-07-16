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

    pub fn login(&self, username: String, password: String) -> Option<User> {
        let storage = Storage::new();
        let mut user = storage.get_user_by_username(username);
        if let Ok(mut user) = user {
            if user.login(user.id, password) {
                // update user
                storage.update_user(user.clone()).expect("update user failed");
                return Some(user);
            }
        }
        None
    }

    pub fn create_teacher(&self, teacher: Teacher) {
        let storage = Storage::new();
        storage.create_teacher(teacher).expect("create teacher failed");
    }

    pub fn create_student(&self, student: Student) {
        let storage = Storage::new();
        storage.create_student(student).expect("create student failed");
    }

    pub fn create_user(&self, mut user: User) {
        let storage = Storage::new();
        user.set_password(user.password.clone());
        storage.create_user(user).expect("create user failed");
    }

    pub fn logout(&self) {
        // unimplemented!("Logout");
    }
}
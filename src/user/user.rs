#[derive(Debug, Clone)]
pub enum Role {
    Student,
    Teacher,
    Admin,
}

// 定义角色结构体, 老师、学生、管理员
#[derive(Debug, Clone)]
pub struct User {
    pub id: u8,
    pub role: Role,
    pub username: String,
    pub last_login: String,
    pub password: String,
    pub create_time: String,
    pub modify_time: String,
}

impl User {
    pub fn new(id: u8, role: Role, username: String, last_login: String, password: String) -> User {
        User {
            id,
            role,
            username,
            last_login,
            password,
            create_time: String::from("2020-01-01"),
            modify_time: String::from("2020-01-01"),
        }
    }

    pub fn login(&mut self, id: u8, password: String) -> bool {
        if self.id == id && self.password == password {
            true
        } else {
            false
        }
    }
}

use std::fmt::Display;
use std::fmt;
use chrono::Local;
use rusqlite::types::FromSql;
use sha256::digest;


#[derive(Debug, Clone)]
pub enum Role {
    Student = 0,
    Teacher,
    Admin,
}

impl FromSql for Role {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(s) => {
                match s {
                    b"Student" => Ok(Role::Student),
                    b"Teacher" => Ok(Role::Teacher),
                    b"Admin" => Ok(Role::Admin),
                    _ => Err(rusqlite::types::FromSqlError::InvalidType),
                }
            }
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Student => write!(f, "Student"),
            Role::Teacher => write!(f, "Teacher"),
            Role::Admin => write!(f, "Admin"),
        }
    }
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
    pub fn new(id: u8, role: Role, username: String, last_login: String, password: String, create_time: String, modify_time: String ) -> User {
        User {
            id,
            role,
            username,
            last_login,
            password,
            create_time,
            modify_time
        }
    }

    pub fn login(&mut self, id: u8, password: String) -> bool {
        // sha256 password
        let input = String::from(password);
        let hashed_password = digest(input);
        if self.id == id && self.password == hashed_password {
            // 当前时间
            let now = Local::now();
            self.last_login = now.format("%Y-%m-%d %H:%M:%S").to_string();
            true
        } else {
            false
        }
    }
    // 设置密码
    pub fn set_password(&mut self, password: String){
        let input = String::from(password);
        let hashed_password = digest(input);
        self.password = hashed_password;

    }

    pub fn logout(&self) {
        unimplemented!("Logout");
    }

}

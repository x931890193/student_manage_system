use std::path;
use crate::user::User;
use crate::teacher::Teacher;
use crate::student::Student;
use crate::curse::Curse;
use crate::class::Class;

// 使用本地文件sql_lite作为存储 定义连接
use rusqlite::{Connection, Result};

// 定义存储结构体
pub struct Storage {
    pub users: Vec<User>,
    pub teachers: Vec<Teacher>,
    pub students: Vec<Student>,
    pub courses: Vec<Curse>,
    pub classes: Vec<Class>,
}

// 定义存储结构体的实现
impl Storage {
    // 初始化sql_lite连接
    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }
    pub fn add_teacher(&mut self, teacher: Teacher) {
        self.teachers.push(teacher);
    }
    pub fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }
    pub fn add_course(&mut self, course: Curse) {
        self.courses.push(course);
    }
    pub fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }
    pub fn get_user(&self, id: u8) -> Option<&User> {
        for user in &self.users {
            if user.id == id {
                return Some(user);
            }
        }
        None
    }
    pub fn get_teacher(&self, id: u8) -> Option<&Teacher> {
        for teacher in &self.teachers {
            if teacher.id == id {
                return Some(teacher);
            }
        }
        None
    }
    pub fn get_student(&self, id: u8) -> Option<&Student> {
        for student in &self.students {
            if student.id == id {
                return Some(student);
            }
        }
        None
    }
    pub fn get_course(&self, id: u8) -> Option<&Curse> {
        for curse in &self.courses {
            if curse.id == id {
                return Some(curse);
            }
        }
        None
    }
    pub fn get_class(&self, id: u8) -> Option<&Class> {
        for class in &self.classes {
            if class.id == id {
                return Some(class);
            }
        }
        None
    }
    pub fn get_user_mut(&mut self, id: u8) -> Option<&mut User> {
        for user in &mut self.users {
            if user.id == id {
                return Some(user);
            }
        }
        None
    }
    pub fn get_teacher_mut(&mut self, id: u8) -> Option<&mut Teacher> {
        for teacher in &mut self.teachers {
            if teacher.id == id {
                return Some(teacher);
            }
        }
        None
    }
    pub fn get_student_mut(&mut self, id: u8) -> Option<&mut Student> {
        for student in &mut self.students {
            if student.id == id {
                return Some(student);
            }
        }
        None
    }
    pub fn get_course_mut(&mut self, id: u8) -> Option<&mut Curse> {
        for curse in &mut self.courses {
            if curse.id == id {
                return Some(curse);
            }
        }
        None
    }
    pub fn get_class_mut(&mut self, id: u8) -> Option<&mut Class> {
        for class in &mut self.classes {
            if class.id == id {
                return Some(class);
            }
        }
        None
    }
    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }
    pub fn get_teachers(&self) -> &Vec<Teacher> {
        &self.teachers
    }
    pub fn get_students(&self) -> &Vec<Student> {
        &self.students
    }
    pub fn get_courses(&self) -> &Vec<Curse> {
        &self.courses
    }
    pub fn get_classes(&self) -> &Vec<Class> {
        &self.classes
    }
    pub fn get_users_mut(&mut self) -> &mut Vec<User> {
        &mut self.users
    }
    pub fn get_teachers_mut(&mut self) -> &mut Vec<Teacher> {
        &mut self.teachers
    }
    pub fn get_students_mut(&mut self) -> &mut Vec<Student> {
        &mut self.students
    }
    pub fn get_courses_mut(&mut self) -> &mut Vec<Curse> {
        &mut self.courses
    }
    pub fn get_classes_mut(&mut self) -> &mut Vec<Class> {
        &mut self.classes
    }
}


// 连接sqllite 导入sql_lite
pub fn connect() -> Result<Connection> {
    // 如果文件不存在则创建 创建数据库，初始化数据
    if !path::Path::new(".db.sqlite").exists() {
        let conn = Connection::open(".db.sqlite")?;
        // 创建User
        conn.execute(
            "CREATE TABLE user (
                id              INTEGER PRIMARY KEY,
                role            INTEGER NOT NULL,
                username        TEXT NOT NULL,
                last_login      TEXT NOT NULL,
                password        TEXT NOT NULL
            )",
            [],
        )?;
        // 创建Teacher
        conn.execute(
            "CREATE TABLE teacher (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                age             INTEGER NOT NULL
                curses          TEXT NOT NULL
            )",
            [],
        )?;
        // 创建Student
        conn.execute(
            "CREATE TABLE student (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                age             INTEGER NOT NULL,
                class           TEXT NOT NULL,
                curses          TEXT NOT NULL,
                number          INTEGER NOT NULL
            )",
            [],
        )?;
        // 创建Course
        conn.execute(
            "CREATE TABLE course (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                description     TEXT NOT NULL,
                effect          TEXT NOT NULL
            )",
            [],
        )?;

        return Ok(conn);
    }


    return Connection::open(".db.sqlite");

}
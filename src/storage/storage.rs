use std::path;
use chrono::Local;
use crate::user::User;
use crate::teacher::Teacher;
use crate::student::Student;
use crate::curse::Curse;
use crate::class::Class;

// 使用本地文件sql_lite作为存储 定义连接
use rusqlite::{Connection, Error, Result};

// 定义存储结构体
#[derive(Debug, Clone)]
pub struct Storage;


// 连接sqllite 导入sql_lite
fn connect() -> Result<Connection> {
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
                password        TEXT NOT NULL,
                create_time     TEXT NOT NULL,
                modify_time     TEXT NOT NULL
            )",
            [],
        )?;
        // 创建Teacher
        conn.execute(
            "CREATE TABLE teacher (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                age             INTEGER NOT NULL,
                create_time     TEXT NOT NULL,
                modify_time     TEXT NOT NULL
            )",
            [],
        )?;
        // 创建Student
        conn.execute(
            "CREATE TABLE student (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                age             INTEGER NOT NULL,
                number          INTEGER NOT NULL,
                create_time     TEXT NOT NULL,
                modify_time     TEXT NOT NULL
            )",
            [],
        )?;
        // 创建Course
        conn.execute(
            "CREATE TABLE course (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                description     TEXT NOT NULL,
                effect          TEXT NOT NULL,
                create_time     TEXT NOT NULL,
                modify_time     TEXT NOT NULL
            )",
            [],
        )?;
        // 创建Class
        conn.execute(
            "CREATE TABLE class (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                create_time     TEXT NOT NULL,
                modify_time     TEXT NOT NULL
            )",
            [],
        )?;
        // 创建TeacherClass
        conn.execute(
            "CREATE TABLE teacher_class (
                id              INTEGER PRIMARY KEY,
                teacher_id      INTEGER NOT NULL,
                class_id        INTEGER NOT NULL,
                create_time     TEXT NOT NULL,
                modify_time     TEXT NOT NULL
            )",
            [],
        )?;
        // 创建StudentClass
        conn.execute(
            "CREATE TABLE student_class (
                id              INTEGER PRIMARY KEY,
                student_id      INTEGER NOT NULL,
                class_id        INTEGER NOT NULL,
                create_time     TEXT NOT NULL,
                modify_time     TEXT NOT NULL
            )",
            [],
        )?;
        // 创建CurseStudent
        conn.execute(
            "CREATE TABLE curse_student (
                id              INTEGER PRIMARY KEY,
                curse_id        INTEGER NOT NULL,
                student_id      INTEGER NOT NULL,
                create_time     TEXT NOT NULL,
                modify_time     TEXT NOT NULL
            )",
            [],
        )?;
        // 创建CurseTeacher
        conn.execute(
            "CREATE TABLE curse_teacher (
                id              INTEGER PRIMARY KEY,
                curse_id        INTEGER NOT NULL,
                teacher_id      INTEGER NOT NULL,
                create_time     TEXT NOT NULL,
                modify_time     TEXT NOT NULL
            )",
            [],
        )?;

        return Ok(conn);
    }

    return Connection::open(".db.sqlite");

}

impl Storage {
    pub fn new() -> Storage {
        Storage
    }

    pub fn get_user_by_username(&self, username: String) -> Result<User, Error> {
        let conn = connect().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM user WHERE username = ?1").unwrap();
        let user_iter = stmt.query_map([username], |row| {
            Ok(User::new(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
                row.get(5).unwrap(),
                row.get(6).unwrap(),

            ))
        }).unwrap();
        for user in user_iter {
            return user;
        }
        return Err(Error::QueryReturnedNoRows);
    }

    pub fn get_user_by_id(&self, id: u8) -> User {
        let conn = connect().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM user WHERE id = ?1").unwrap();
        let user_iter = stmt.query_map([id], |row| {
            Ok(User::new(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
                row.get(5).unwrap(),
                row.get(6).unwrap(),
            ))
        }).unwrap();

        for user in user_iter {
            return user.unwrap();
        }

        unimplemented!("Get user by id");
    }

    pub fn get_teacher_by_id(&self, id: u8) -> Teacher {
        let conn = connect().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM teacher WHERE id = ?1").unwrap();
        let teacher_iter = stmt.query_map([id], |row| {
            Ok(Teacher::new(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
            ))
        }).unwrap();

        for teacher in teacher_iter {
            return teacher.unwrap();
        }

        unimplemented!("Get teacher by id");
    }

    pub fn get_student_by_id(&self, id: u8) -> Student {
        let conn = connect().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM student WHERE id = ?1").unwrap();
        let student_iter = stmt.query_map([id], |row| {
            Ok(Student::new(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
                row.get(5).unwrap(),
            ))
        }).unwrap();

        for student in student_iter {
            return student.unwrap();
        }

        unimplemented!("Get student by id");
    }

    pub fn update_user(&self, user: User) -> Result<(), Error> {
        let conn = connect().unwrap();
        let mut stmt = conn.prepare("UPDATE user SET last_login = ?1 WHERE id = ?2").unwrap();
        stmt.execute([user.last_login, user.id.to_string()])?;
        Ok(())
    }

    pub fn create_user(&self, user: User) -> Result<(), Error> {
        let conn = connect().unwrap();
        let now = Local::now();
        let now = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let mut stmt = conn.prepare("INSERT INTO user (role, username, last_login, password, create_time, modify_time) VALUES (?1, ?2, ?3, ?4, ?5, ?6)").unwrap();
        stmt.execute([user.role.to_string(), user.username, user.last_login, user.password, now.clone(), now.clone()])?;
        Ok(())
    }

    pub fn create_teacher(&self, teacher: Teacher) -> Result<(), Error> {
        let conn = connect().unwrap();
        let now = Local::now();
        let now = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let mut stmt = conn.prepare("INSERT INTO teacher (name, age, create_time, modify_time) VALUES (?1, ?2, ?3, ?4)").unwrap();
        stmt.execute([teacher.name, teacher.age.to_string(), now.clone(), now.clone()])?;
        Ok(())
    }

    pub fn create_student(&self, student: Student) -> Result<(), Error> {
        let conn = connect().unwrap();
        let now = Local::now();
        let now = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let mut stmt = conn.prepare("INSERT INTO student (name, age, number, create_time, modify_time) VALUES (?1, ?2, ?3, ?4, ?5)").unwrap();
        stmt.execute([student.name, student.age.to_string(), student.number.to_string(), now.clone(), now.clone()])?;
        Ok(())
    }

    pub fn create_curse(&self, curse: Curse) -> Result<(), Error> {
        let conn = connect().unwrap();
        let now = Local::now();
        let now = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let mut stmt = conn.prepare("INSERT INTO curse (name, description, effect, create_time, modify_time) VALUES (?1, ?2, ?3)").unwrap();
        stmt.execute([curse.name, curse.description, curse.effect.to_string(), now.clone(), now.clone()])?;
        Ok(())
    }

    pub fn create_class(&self, class: Class) -> Result<(), Error> {
        let conn = connect().unwrap();
        let now = Local::now();
        let now = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let mut stmt = conn.prepare("INSERT INTO class (name, create_time, modify_time) VALUES (?1, ?2, ?3)").unwrap();
        stmt.execute([class.name, now.clone(), now.clone()])?;
        Ok(())
    }
}



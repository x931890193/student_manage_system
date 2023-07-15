use crate::student::Student;
use crate::teacher::Teacher;
use serde::{Deserialize, Serializer, Deserializer};

#[derive(Debug)]
pub struct Class {
    pub id: u8,
    pub name: String,
    pub teacher: String,
    pub students: Vec<Student>,
    pub create_time: String,
    pub modify_time: String,

}

#[derive(Debug)]
pub struct TeacherClass {
    pub id: u8,
    pub teacher_id: u8,
    pub class_id: u8,
}

#[derive(Debug)]
pub struct StudentClass {
    pub id: u8,
    pub student_id: u8,
    pub class_id: u8,
}

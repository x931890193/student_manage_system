

#[derive(Debug)]
pub struct Curse {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub effect: u8,
    pub create_time: String,
    pub modify_time: String,
}

#[derive(Debug)]
pub struct CurseStudent {
    pub id: u8,
    pub curse_id: u8,
    pub student_id: u8,
    pub create_time: String,
    pub modify_time: String,
}

#[derive(Debug)]
pub struct CurseTeacher {
    pub id: u8,
    pub curse_id: u8,
    pub teacher_id: u8,
    pub create_time: String,
    pub modify_time: String,
}

impl Curse {
    pub fn new(id: u8, name: String, description: String, effect: u8, create_time: String, modify_time: String) -> Curse {
        Curse {
            id,
            name,
            description,
            effect,
            create_time,
            modify_time,
        }
    }
}
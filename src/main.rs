use crate::user::Role;

mod class;
mod student;
mod teacher;
mod curse;
mod api;
mod storage;
mod user;

fn main() {
    let api = api::API::new();
    let user = api.login("admin".to_string(), "admin".to_string());
    if let Some(user) = user {
        println!("user: {:?}", user);
        match user.role {
            Role::Student => {
                println!("welcome student");
            },
            Role::Teacher => {
                println!("welcome teacher");
            },
            Role::Admin => {
                println!("welcome admin");
            },
        }
    }else {
        println!("user not found exit!");
        print!("create admin user");

        let user = user::User::new(1, Role::Admin, "admin".to_string(), "2020-01-01".to_string(), "admin".to_string(), "2020-01-01".to_string(),"2020-01-01".to_string() );
        api.create_user(user);

    }

}

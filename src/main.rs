mod class;
mod student;
mod teacher;
mod curse;
mod api;
mod storage;
mod user;

fn main() {
    println!("Hello, world!");
    let servive = api::API::new();
    println!("servive: {:?}", servive.login("admin".to_string(), "admin".to_string()));
    println!("servive: {:?}", servive.logout());

}

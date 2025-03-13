use crate::other::say_hello;

pub struct User{
    pub username: String,
    pub email: String,
    pub age: u8
}

impl User{
    pub fn register(&self){
        println!("User {} registered", self.username);
        say_hello(&self.username);
    }

    pub fn get_data(&self){
        println!("Username: {}, Email: {}, Age: {}", self.username, self.email, self.age);
    }
}
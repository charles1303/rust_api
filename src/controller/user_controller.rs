use service::user_service;
use std::vec;
use model::user::User;

pub fn get_users() -> vec::Vec<User>{
    user_service::get_users()
}

pub fn create_user(user: User) -> User{
    user_service::create_user(&user)
}
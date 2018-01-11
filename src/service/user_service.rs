use model::user::User;
use dao::user_dao;
use std::vec;

pub fn get_users() -> vec::Vec<User>{
    user_dao::get_users()
}

pub fn create_user(user: User) -> User{
    let mut u = try!(user_dao::create_user(&user));
    Ok(u)
}
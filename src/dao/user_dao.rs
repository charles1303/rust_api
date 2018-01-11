use dao::connection;
use model::user::User;

use dao::schema::*;

use dao::schema::user::dsl::*;

use diesel::prelude::*;

use std::vec;


pub fn get_users() -> vec::Vec<User>{
    
    
    let conn = connection::establish_connection();
    let results = user.limit(5)
        .load::<User>(&conn)
        .expect("Error loading users");

    results
}

pub fn create_user(user: User) -> User{
    
    let conn = connection::establish_connection();
    
    diesel::insert_into(users::table)
        .values(&user)
        .get_result(&conn)
        .expect("Error saving new user")

    user
}


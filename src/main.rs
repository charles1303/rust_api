#[macro_use]
extern crate nickel;

extern crate serde;
extern crate serde_json;

extern crate rustc_serialize;
use rustc_serialize::json::{self,Json, Parser};

extern crate api;

use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};
use serde_json::Error;
use nickel::status::StatusCode::{self};

use api::controller::user_controller;

fn main() {

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/users", middleware! { |request, mut response|

        let users = user_controller::get_users();
        let json = json::encode(&users).unwrap();
        
        response.set(MediaType::Json);
        response.set(StatusCode::Ok);
        json
    
    });

    router.post("/users/new", middleware! { |request, mut response|

        let user = User{
            username : "username",
            password : "password",
            firstname : "firstname",
            lastname : "lastname",
            email : "email"
        };
        let user = user_controller::create_user(&user);
        let json = json::encode(&user).unwrap();

        response.set(MediaType::Json);
        response.set(StatusCode::Ok);
        json


    });

    router.delete("/users/:id", middleware! { |request, response|

        format!("Hello from DELETE /users/:id")

    });

    server.utilize(router);

    server.listen("127.0.0.1:9000");
}
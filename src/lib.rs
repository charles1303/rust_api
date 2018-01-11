#![recursion_limit="128"]

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

extern crate rustc_serialize;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate dotenv;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_infer_schema;

extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate lazy_static;


/*#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}*/

pub mod dao;

pub mod model;

pub mod service;

pub mod controller;
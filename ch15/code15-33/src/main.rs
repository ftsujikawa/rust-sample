use diesel::prelude::*;
#[macro_use]
extern crate diesel;
#[derive(Debug, Queryable)]
struct User {
    id: i32,
    name: String,
    age: i32,
}
table! {
    users (id) {
        id -> Integer,
        name -> Text,
        age -> Integer,
    }
}
use self::users::dsl::*;
fn main() {
    println!("Hello, world!");
}

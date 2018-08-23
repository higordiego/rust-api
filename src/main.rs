#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{Json, Value};

mod user;
use user::{User};

#[post("/", data = "<user>")]
fn create(user: Json<User>) -> Json<User> {
    user
}

#[get("/")]
fn read() -> Json<Value> {
    Json(json!({"user": "1"}))
}

#[put("/<id>", data ="<user>")]
fn put(id: i32, user: Json<User>) -> Json<User> {
    user
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<Value> {
    Json(json!({"status": "Ok!"}))
}



fn main() {
    rocket::ignite()
        .mount("/", routes![create, read, put, delete])
        .launch();
}
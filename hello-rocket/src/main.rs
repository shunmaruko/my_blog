#![feature(proc_macro_hygiene, decl_macro)]
//it enables nightly builds

#[macro_use] //import all macros
extern crate rocket;
use rocket::http::RawStr;

#[get("/")]
fn hello() -> &'static str {
    "Hello, Rust 2018!"
}

#[get("/hello/<name>")]
fn hi(name: String) -> String {
    name
}

#[get("/user/<id>")]
fn user(id: usize) { /**/ }

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) { /* */ }

#[get("/user/<id>", rank = 3)]
fn user_str(id: &RawStr) {/* */ }

fn main() {
    rocket::ignite()
        .mount("/", routes![hello, hi, user, user_int, user_str])
        .launch();
}

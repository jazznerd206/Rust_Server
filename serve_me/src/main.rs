#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Don't be suspicious..."
}

mod other {
    #[get("/world")]
    pub fn world() -> &'static str {
        "Hello, poopie world!"
    }
}

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, outside world!"
}

fn main() {
    // rocket::ignite().mount("/", routes![index]).launch();
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/hello", routes![hello, other::world])
        .launch();
}
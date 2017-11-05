#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate uuid;
extern crate rocket;

mod paste_id;
mod upload;

#[get("/")]
fn index() -> &'static str {
    "Welcome to your bookmarking server"
}

fn main() {
    rocket::ignite().mount("/", routes![index, upload::upload, upload::retrieve]).launch();
}

#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use routes::*;

mod models;
mod routes;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index, get_user, get_user_by_user_id, post_user],
        )
        .launch();
}

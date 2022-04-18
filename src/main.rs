#[macro_use]
extern crate rocket;

use rusqlite;
use rocket::{Rocket,Build};
use rocket::fs::{relative, FileServer};
use lazy_static::lazy_static;

/*use crate::webapi::user::{
    ...
};*/

mod webapi;
mod error;
use crate::webapi::{index,server_js,server_wasm};
use crate::webapi::challenge::{post_challenge};

lazy_static! {
    static ref SQLITE_FILE_AUTH: String = String::from("data/auth.db");
}

#[launch]
fn rocket() -> Rocket<Build> {
    let path = SQLITE_FILE_AUTH.clone();
    let conn = rusqlite::Connection::open(&path).expect("SQLITE file not found");

    if let Err((_, e)) = conn.close() {
        println!("{}", e);
    };

    rocket::build()
        .mount("/", FileServer::from(relative!("static/forms")))
        .mount(
            "/",
            routes![
                index,
                server_js,
                server_wasm,

                post_challenge,
            ],
        )
}

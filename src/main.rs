#[macro_use]
extern crate rocket;
#[macro_use]
extern crate mysql;

use rocket::{Rocket,Build};
use rocket::fs::{relative, FileServer};
use lazy_static::lazy_static;

/*use crate::webapi::user::{
    ...
};*/

mod webapi;
mod error;
mod sql;
use crate::webapi::{index,server_js,server_wasm,logout};
use crate::webapi::challenge::{login};
use crate::sql::mysql::request::{};
use crate::sql::mysql::schema::{User,Challenge,Cookie};

lazy_static! {
    static ref MYSQL: String = String::from("mysql");
    static ref MYSQL_DB: String = String::from("cheap");
    // Using mysql in a container but running the binary on the host so impossible to use docker' dns 
    static ref MYSQL_HOST: String = String::from("172.20.0.2");
    // gonna remove it (far) later

    static ref SQL_HEAD: String = format!("{}",MYSQL.to_string());
    static ref SQL_TAIL: String = format!("{}/{}",MYSQL_HOST.to_string(),MYSQL_DB.to_string());

    static ref SQL_ROOT_USER: String = String::from("Camel");
    static ref SQL_REGISTER_USER: String = String::from("Camel");
    static ref SQL_AUTH_USER: String = String::from("Camel");

    static ref SQL_ROOT: String = format!("{}://{}:8Fish8@{}",SQL_HEAD.to_string(),SQL_ROOT_USER.to_string(),SQL_TAIL.to_string());
    static ref SQL_REGISTER: String = format!("{}://{}:8Fish8@{}",SQL_HEAD.to_string(),SQL_REGISTER_USER.to_string(),SQL_TAIL.to_string());
    static ref SQL_AUTH: String = format!("{}://{}:8Fish8@{}",SQL_HEAD.to_string(),SQL_AUTH_USER.to_string(),SQL_TAIL.to_string());
}

#[launch]
fn rocket() -> Rocket<Build> {
    /*let path = SQLITE_FILE_AUTH.clone();
    let conn = rusqlite::Connection::open(&path).expect("SQLITE file not found");

    if let Err((_, e)) = conn.close() {
        println!("{}", e);
    };
    */

    rocket::build()
        .mount("/", FileServer::from(relative!("static/forms")))
        .mount(
            "/",
            routes![
                index,
                server_js,
                server_wasm,

                login,
                logout,
            ],
        )
}

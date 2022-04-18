use crate::SQLITE_FILE_AUTH;

pub mod challenge;
pub mod cookie;

use crate::error::CHEAPError;
//use crate::webapi::user::User;

use rocket::http::Status; // https://api.rocket.rs/v0.4/rocket/http/struct.Status.html#structfield.reason
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use rocket::form::Form;

#[get("/")]
pub async fn index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("static/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/js/<ressource>")]
pub async fn server_js(ressource: &str) -> Result<NamedFile, NotFound<String>> {
    NamedFile::open(format!("static/js/{}",ressource))
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/wasm/<ressource>")]
pub async fn server_wasm(ressource: &str) -> Result<NamedFile, NotFound<String>> {
    NamedFile::open(format!("static/wasm/{}",ressource))
        .await
        .map_err(|e| NotFound(e.to_string()))
}




#[post("/", data = "<args>")]
pub fn index_post(args: String) -> String {
    println!("{}",args);
    args
}
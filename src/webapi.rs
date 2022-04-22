pub mod challenge;
pub mod cookie;

use crate::error::CHEAPError;
//use crate::webapi::user::User;

use rocket::http::{Status,Cookie,CookieJar}; // https://api.rocket.rs/v0.5-rc/rocket/http/struct.Status.html
use rocket::fs::NamedFile;
use rocket::response::{Redirect,Flash,status::NotFound};
use rocket::form::Form;

use time::{OffsetDateTime,Duration};

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

#[post("/logout")]
pub fn logout(jar: &CookieJar<'_>) -> Flash<Redirect> {
    //println!("{:#?}",jar.get_private("user_id"));
    jar.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to(uri!(index)), "Successfully logged out.")
}
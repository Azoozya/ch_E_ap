//use crate::webapi::User;

use crate::webapi::SQLITE_FILE_AUTH;

use crate::webapi::Form;
use crate::webapi::Status;

use crate::webapi::CHEAPError;

#[derive(FromForm)]
pub struct Challenge {
    stage: i32,
    // will replace String by User type
    user: String,
    signed: String,
}

#[post("/challenge", data = "<args>")]
pub fn post_challenge(args: String) -> String {
    println!("{}",args);
    args
}
//use crate::webapi::User;

use crate::SQL_AUTH;

use crate::webapi::CHEAPError;

use crate::webapi::{ Form, Status, Redirect };
use crate::webapi::{ Cookie, CookieJar };
use crate::webapi::{ Duration, OffsetDateTime };

use crate::webapi::cryptography::{import_and_verify};

#[derive(FromForm, Debug)]
pub struct Challenge {
    stage: i32,
    // will replace String by User type
    user: String,
    signed: String,
}

impl Challenge {
    pub fn stage(&self) -> i32 {
        self.stage
    }

    pub fn user(&self) -> String {
        //user: self.user(), -> User
        self.user.clone()
    }

    pub fn signed(&self) -> String {
        self.signed.clone()
    }

    #[allow(dead_code)]
    pub fn clean(&mut self) -> Result<(), CHEAPError> {
        if self.stage < 1 || self.stage > 2 {
            return Err(CHEAPError::InvalidStage);
        }
        //self.user.clean()?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn new(stage: i32, user: String, signed: String) -> Challenge {
        Challenge {
            stage,
            //user: User::new(user),
            user: user.clone(),
            signed: signed.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn clone(&self) -> Challenge {
        Challenge {
            stage: self.stage(),
            user: self.user(),
            signed: self.signed(),
        }
    }
}

#[post("/login", data = "<chlg>")]
pub fn login(jar: &CookieJar<'_>, mut chlg: Form<Challenge>) -> (Status, &'static str) {
    let chlg = match chlg.clean() {
        Err(e) => {
            if cfg!(debug_assertions) {
                println!("{:#?}", e);
            }
            return (e.to_status(), "");
        }
        Ok(_) => chlg.clone(),
    };
    println!("{:#?}", chlg);

    match chlg.stage {
        1 => stage1(chlg),
        2 => stage2(jar, chlg),
        _ => (CHEAPError::Unknown.to_status(), ""),
    }
}

fn stage1(chlg: Challenge) -> (Status, &'static str) {
    // Check if user is a known user
    //chlg.user.name()
    if chlg.user == String::from("lama") {
        (Status::Ok, "12345")
    } else {
        (CHEAPError::InvalidUser.to_status(), "/")
    }
}

fn stage2(jar: &CookieJar<'_>, chlg: Challenge) -> (Status, &'static str) {
    if import_and_verify(chlg,String::from("12345")) {
        let mut cookie = Cookie::new("user_id", "Welcome !");
        // None => Session cookie
        cookie.set_expires(None);

        let offset = OffsetDateTime::now_utc() + Duration::minutes(15);        

        //Store expiration time
        println!("{:#?}", offset.timestamp());

        //No need to store cookie, if get_private call return nothing it means cookie has been altered
        jar.add_private(cookie);

        (Status::Ok, "Access Granted :3")
    } else {
        (CHEAPError::InvalidSigned.to_status(), "incorrect signed")
    }
}

#[get("/logged")]
pub fn logged(jar: &CookieJar<'_>) -> Redirect {
    match jar.get_private("user_id") {
        None => Redirect::to(uri!("/challenge.html")),
        Some(_) => Redirect::to(uri!("/")),
    }
}

/*
 * id: autoincremented uid
 * name: clients will supply a string, then we'll translate it into id
 * pubkey: authentification passwordless => with asymetric cryptography
 */
#[derive(Debug)]
pub struct User {
    id: u32,
    name: String,
    pubkey: String,
}

/*
 * user_id/id: each 'challenge' is dedicated to one specific User
 * nonce: random number which will be the main content of the challenge
 * expire: unix_timestamp defining a time limit to pass the challenge
 */
#[derive(Debug)]
pub struct Challenge {
    user_id: u32,
    nonce: u32,
    expire: u64,
}

/*
 * user_id/id: each 'cookie' is dedicated to one specific User
 * expire: unix_timestamp defining a time limit when accept the cookie.
 *         also , retain last activity, for delete unused Users.
 */
#[derive(Debug)]
pub struct Cookie {
    user_id: u32,
    expire: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CHEAPTable {
    Users,
    Challenges,
    Cookies,
}

impl CHEAPTable {
    pub fn name(&self) -> String {
        match self {
            CHEAPTable::Users => String::from("Users"),
            CHEAPTable::Challenges => String::from("Challenges"),
            CHEAPTable::Cookies => String::from("Cookies"),
            _ => String::from("Unknown"),
        }
    }
}

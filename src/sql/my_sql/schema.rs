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
 * user_id/id: each 'Session' is dedicated to one specific User
 * expire: unix_timestamp defining a time limit when accept the session's cookie.
 *         also , retain last expiracy, for delete unused Users.
 */
#[derive(Debug)]
pub struct Session {
    user_id: u32,
    expire: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CHEAPTable {
    Users,
    Challenges,
    Sessions,
}

impl CHEAPTable {
    pub fn name(&self) -> String {
        match self {
            CHEAPTable::Users => String::from("Users"),
            CHEAPTable::Challenges => String::from("Challenges"),
            CHEAPTable::Sessions => String::from("Cookies"),
            _ => String::from("Unknown"),
        }
    }
}

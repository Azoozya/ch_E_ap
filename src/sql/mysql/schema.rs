/*
 * id: autoincremented uid
 * name: clients will supply a string, then we'll translate it into id 
 * pubkey: authentification passwordless => with asymetric cryptography
 */

pub struct User {
    id: u32,
    name: String,
    pubkey: String,
}

/*
 * user_id/id: each 'challenge' is dedicated to one specific User
 * nonce: random number which will be the main content of the challenge
 * expire: unix_timstamp defining a time limit to pass the challenge
 */
pub struct Challenge {
    user_id: u32,
    nonce: u32,
    expire: u64,
}


/*
 * user_id/id: each 'cookie' is dedicated to one specific User
 * expire: unix_timstamp defining a time limit to accept the cookie.
 *         also , retain last activity, for clean database from unused Users.
 */
pub struct Cookie {
    user_id: u32,
    expire: u64,
}
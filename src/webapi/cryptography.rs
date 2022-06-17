use crate::r_crypt::PublicKey;
use crate::r_crypt::keypair::KeyPair;
use crate::webapi::challenge::Challenge;

/**********************************************************/

//nonce will be retrieved from db, later
pub fn import_and_verify(chlg: Challenge, nonce: String) -> bool {
    match import_from_file(chlg.user()){
        None => false,
        Some(public_key) => match verify(public_key,nonce,chlg.signed()){
            None => false,
            Some(_) => true,
        },
    }
}

fn import_from_file(user: String) -> Option<PublicKey> {
    match PublicKey::read_openssh_file(std::path::Path::new("data/dev.pub")) {
        Err(_) => None,
        Ok(public_key) => Some(public_key),
    }
}

/**********************************************************/

// Nonce is retrieve from db
pub fn load_and_verify(chlg: Challenge) -> bool {
    match import_from_db(chlg.user()){
        (None,_) => false,
        (Some(public_key),nonce) => match verify(public_key,nonce,chlg.signed()){
            None => false,
            Some(_) => true,
        },
    }
}
fn import_from_db(user: String) -> (Option<PublicKey>,String) {
    (None,String::from(""))
}

/**********************************************************/


fn verify(key: PublicKey, data: String, signature: String) -> Option<()> {
    let verifier = KeyPair::new(key.algorithm());
    verifier.verify(Vec::from(data),Vec::from(signature),Some(key))
}
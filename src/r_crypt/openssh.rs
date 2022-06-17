use crate::r_crypt::{ PrivateKey, PublicKey, OsRng };
use crate::{ KeyPair, Algorithm, LineEnding };

pub fn openssh_export(
    keypair: KeyPair,
    passphrase: Vec<u8>,
    ending: LineEnding,
) -> Option<(Vec<u8>, Vec<u8>)> {
    /**** Public ****/
    let mut public: PublicKey = match keypair.public() {
        None => {
            return None;
        }
        Some(key) => key,
    };

    // Set an arbitrary comment, otherwise import will fail
    public.set_comment("r_cryp@wasm");

    // Encode
    let public_file = match public.to_openssh() {
        Err(_) => {
            return None;
        }
        Ok(public_file) => Vec::from(public_file.as_bytes()),
    };

    /**** Private ****/
    let mut private: PrivateKey = match keypair.private() {
        None => {
            return None;
        }
        Some(key) => key,
    };

    // Encrypt if a passphrase have been submitted (correctly)
    if passphrase.len() > 0 {
        private = match private.encrypt(&mut OsRng, passphrase.as_slice()) {
            Err(_) => {
                return None;
            }
            Ok(private_encrypted) => private_encrypted,
        };
    }

    // Encode
    let private_file = match private.to_openssh(ending) {
        Err(_) => {
            return None;
        }
        Ok(private_file) => Vec::from(private_file.as_bytes()),
    };

    /**** Result ****/
    Some((public_file, private_file))
}

pub fn openssh_import_private(private_file: Vec<u8>, passphrase: Option<Vec<u8>>) -> Option<KeyPair> {
    let mut private: PrivateKey = match PrivateKey::from_openssh(&private_file) {
        Err(_) => {
            return None;
        }
        Ok(maybe_encrypted) => maybe_encrypted,
    };

    if private.is_encrypted() {
        match passphrase {
            None => { return None; }
            Some(passphrase) => {
                private = match private.decrypt(&passphrase) {
                    Err(_) => {
                        return None;
                    }
                    Ok(decrypted) => decrypted,
                };
            }
        }
    }

    let mut result: KeyPair = KeyPair::new(private.algorithm());

    let algo: Algorithm = result.algo();
    if algo.is_ed25519() {
        match private.key_data().ed25519() {
            None => None,
            Some(keypair) => match result.set_ed(keypair) {
                None => None,
                Some(_) => Some(result),
            },
        }
    } else if algo.is_ecdsa() {
        match private.key_data().ecdsa() {
            None => None,
            Some(keypair) => match result.set_ecdsa(keypair) {
                None => None,
                Some(_) => Some(result),
            },
        }
    } else if algo.is_rsa() {
        match private.key_data().rsa() {
            None => None,
            Some(keypair) => match result.set_rsa(keypair) {
                None => None,
                Some(_) => Some(result),
            },
        }
    } else {
        None
    }
}

pub fn openssh_import_public(public_file: Vec<u8>) -> Option<PublicKey> {
    match String::from_utf8(public_file) {
        Err(_) => None,
        Ok(public_as_string) => match PublicKey::from_openssh(&public_as_string) {
            Err(_) => None,
            Ok(public) => Some(public),
        }
    }

}

pub fn openssh_import(private_file: Option<Vec<u8>>, public_file: Option<Vec<u8>>, passphrase: Option<Vec<u8>>) -> (Option<KeyPair>,Option<PublicKey>) {
    let mut result : (Option<KeyPair>,Option<PublicKey>) = (None,None);

    if let Some(private_file) = private_file {
        result.0 = openssh_import_private(private_file,passphrase);
    }

    if let Some(public_file) = public_file {
        result.1 = openssh_import_public(public_file);
    }
    
    //crate::alert(&format!("{:#?}",result));
    result
}
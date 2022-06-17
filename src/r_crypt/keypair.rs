use crate::r_crypt::{ RsaKeypair, EcdsaKeypair, Ed25519Keypair, PublicKey, PrivateKey };
use crate::r_crypt::{ Base64Bcrypt, Encoding, Sha512, Digest, Signature};
use crate::r_crypt::OsRng;
use crate::Algorithm;

// https://docs.rs/ssh-key/latest/ssh_key/index.html
// https://docs.rs/ssh-key/latest/ssh_key/private/index.html
// https://docs.rs/ssh-key/latest/ssh_key/private/struct.Ed25519Keypair.html
#[derive(Debug)]
pub struct KeyPair {
    ed25519: Option<Ed25519Keypair>,
    rsa: Option<RsaKeypair>,
    ecdsa: Option<EcdsaKeypair>,
    algo: Algorithm,
}

impl KeyPair {
    pub fn new(algo: Algorithm) -> KeyPair {
        return KeyPair {
            ed25519: None,
            rsa: None,
            ecdsa: None,
            algo,
        };
    }

    pub fn generate(&mut self) {
        if self.algo.is_ed25519() {
            if let None = self.ed25519 {
                self.ed25519 = Some(Ed25519Keypair::random(&mut OsRng));
            }
        } else if self.algo.is_rsa() {
            if let None = self.rsa {
                self.rsa = match RsaKeypair::random(&mut OsRng, 4096) {
                    Err(_) => None,
                    Ok(key) => Some(key),
                }
            }
        } else if self.algo.is_ecdsa() {
            if let None = self.ecdsa {
                self.ecdsa = match EcdsaKeypair::random(&mut OsRng, ssh_key::EcdsaCurve::NistP256) {
                    Err(_) => None,
                    Ok(key) => Some(key),
                }
            }
        } else {
        }
        // dsa is depreciated for openssh... really need to support it ? D:
    }

    pub fn private(&self) -> Option<PrivateKey> {
        if self.algo.is_ed25519() {
            match &self.ed25519 {
                Some(keypair) => Some(PrivateKey::from(keypair.clone())),
                None => None,
            }
        } else if self.algo.is_rsa() {
            match &self.rsa {
                Some(keypair) => Some(PrivateKey::from(keypair.clone())),
                None => None,
            }
        } else if self.algo.is_ecdsa() {
            match &self.ecdsa {
                Some(keypair) => Some(PrivateKey::from(keypair.clone())),
                None => None,
            }
        } else {
            None
        }
        // dsa is depreciated for openssh... really need to support it ? D:
    }

    //To complete
    pub fn private_bytes(&self) -> Option<Vec<u8>> {
        if self.algo.is_ed25519() {
            match &self.ed25519 {
                Some(keypair) => Some(Vec::from(keypair.private.clone().to_bytes())),
                None => None,
            }
        } else if self.algo.is_rsa() {
            match &self.rsa {
                Some(_keypair) => None,
                None => None,
            }
        } else if self.algo.is_ecdsa() {
            match &self.ecdsa {
                Some(_keypair) => None,
                None => None,
            }
        } else {
            None
        }
        // dsa is depreciated for openssh... really need to support it ? D:
    }

    pub fn public(&self) -> Option<PublicKey> {
        if self.algo.is_ed25519() {
            match &self.ed25519 {
                Some(keypair) => Some(PublicKey::from(keypair.public)),
                None => None,
            }
        } else if self.algo.is_rsa() {
            match &self.rsa {
                Some(keypair) => Some(PublicKey::from(keypair.public.clone())),
                None => None,
            }
        } else if self.algo.is_ecdsa() {
            match &self.ecdsa {
                Some(keypair) => Some(match PublicKey::from_bytes(keypair.public_key_bytes()) {
                    Err(_) => {
                        return None;
                    }
                    Ok(key) => key,
                }),
                None => None,
            }
        } else {
            None
        }
        // dsa is depreciated for openssh... really need to support it ? D:
    }

    //To complete
    pub fn public_bytes(&self) -> Option<Vec<u8>> {
        if self.algo.is_ed25519() {
            match &self.ed25519 {
                Some(keypair) => Some(Vec::from(keypair.public.clone().0)),
                None => None,
            }
        } else if self.algo.is_rsa() {
            match &self.rsa {
                Some(_keypair) => None,
                None => None,
            }
        } else if self.algo.is_ecdsa() {
            match &self.ecdsa {
                Some(_keypair) => None,
                None => None,
            }
        } else {
            None
        }
        // dsa is depreciated for openssh... really need to support it ? D:
    }

    /***   Could lead to implement   KeyPair::From<T>  ***/
    pub fn set_ed(&mut self, keypair: &Ed25519Keypair) -> Option<()> {
        if self.algo.is_ed25519() {
            if let None = self.ed25519 {
                self.ed25519 = Some(keypair.clone());
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_ecdsa(&mut self, keypair: &EcdsaKeypair) -> Option<()> {
        if self.algo.is_ecdsa() {
            if let None = self.ecdsa {
                self.ecdsa = Some(keypair.clone());
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_rsa(&mut self, keypair: &RsaKeypair) -> Option<()> {
        if self.algo.is_rsa() {
            if let None = self.rsa {
                self.rsa = Some(keypair.clone());
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    }
    /***   Could lead to implement   KeyPair::From<T>  ***/

    /***   Could lead to implement   KeyPair::Into<T>  ***/
    fn as_dalek(&self) -> Option<ed25519_dalek::Keypair> {
        //available only for Ed25519
        if self.algo.is_ed25519() {
            let secret_key: ed25519_dalek::SecretKey = match self.private_bytes() {
                None => {
                    return None;
                }
                Some(secret_bytes) => match ed25519_dalek::SecretKey::from_bytes(&secret_bytes) {
                    Err(_) => {
                        return None;
                    }
                    Ok(secret_key) => secret_key,
                },
            };

            let public_key: ed25519_dalek::PublicKey = match self.public_bytes() {
                None => {
                    return None;
                }
                Some(public_bytes) => match ed25519_dalek::PublicKey::from_bytes(&public_bytes) {
                    Err(_) => {
                        return None;
                    }
                    Ok(public_key_dalek) => public_key_dalek,
                },
            };

            // Assuming the secret key is correct, otherwise it should have cause an error before
            Some(ed25519_dalek::Keypair {
                secret: ed25519_dalek::SecretKey::from_bytes(&secret_key.to_bytes()).unwrap(),
                public: public_key.clone(),
            })
        } else {
            return None;
        }
    }
    /***   Could lead to implement   KeyPair::Into<T>  ***/

    //To complete
    pub fn sign(&self, data: Vec<u8>) -> Option<Vec<u8>> {
        if self.algo.is_ed25519() {
            let keypair_dalek: ed25519_dalek::Keypair = match self.as_dalek() {
                None => {
                    return None;
                }
                Some(keypair_dalek) => keypair_dalek,
            };

            let mut prehashed: Sha512 = Sha512::new();
            prehashed.update(&data);

            let signed_data: Signature = match keypair_dalek.sign_prehashed(prehashed, None) {
                Err(_) => {
                    return None;
                }
                Ok(signed_data) => signed_data,
            };
            Some(Vec::from(Base64Bcrypt::encode_string(&signed_data.to_bytes())))
            //Some(Vec::from(signed_data.to_bytes()))
        } else {
            None
        }
    }

    // we pass signature to this module through linear memory i.e read a Vec<u8>
    // backends will require a conversion to Signature
    //To complete
    pub fn verify(&self, data: Vec<u8>, signature: Vec<u8>, key: Option<PublicKey>) -> Option<()> {
        if self.algo.is_ed25519() {
            // convert to dalek's publikey

            let public_key: ed25519_dalek::PublicKey =  match key {
                None => {
                    match self.public_bytes() {
                        None => {
                            return None;
                        },
                        Some(public_bytes) => match ed25519_dalek::PublicKey::from_bytes(&public_bytes) {
                            Err(_) => {
                                return None;
                            },
                            Ok(public_key_dalek) => public_key_dalek,
                        },
                    }
                },

                Some(key) => {
                    match key.key_data().ed25519(){
                        None => { return None; },
                        Some(public_bytes) => match ed25519_dalek::PublicKey::from_bytes(&public_bytes.0) {
                            Err(_) => {
                                return None;
                            },
                            Ok(public_key_dalek) => public_key_dalek,
                        },
                    }
                },
            };

            let mut prehashed: Sha512 = Sha512::new();
            prehashed.update(&data);

            // decode base64
            let signature_bytes: Vec<u8> = match std::str::from_utf8(&signature) {
                Err(_) => {
                    return None;
                }
                Ok(signature_str) => match Base64Bcrypt::decode_vec(signature_str) {
                    Err(_) => {
                        return None;
                    }
                    Ok(signature_bytes) => signature_bytes,
                },
            };

            // convert to Signature
            let signed_data: Signature = match Signature::from_bytes(&signature_bytes) {
                Err(_) => {
                    return None;
                }
                Ok(signed_data) => signed_data,
            };

            // verify
            match public_key.verify_prehashed(prehashed, None, &signed_data) {
                Err(_) => None,
                Ok(_) => Some(()),
            }
        } else {
            None
        }
    }

    pub fn algo(&self) -> Algorithm {
        self.algo.clone()
    }
}
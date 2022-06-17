pub mod keypair;
pub mod io;
pub mod openssh;

pub const WASM_MEMORY_BUFFER_SIZE: usize = 10000;

use ssh_key::private::{EcdsaKeypair, Ed25519Keypair, RsaKeypair};
pub use ssh_key::{ PrivateKey, PublicKey};

use ed25519_dalek::{Digest, Sha512, Signature};
use base64ct::{Base64Bcrypt, Encoding};

use rand_core::OsRng;
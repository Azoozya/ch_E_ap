use wasm_bindgen::prelude::*;

mod r_crypt;

use ssh_key::{LineEnding,Algorithm};
use crate::r_crypt::keypair::KeyPair;
use crate::r_crypt::openssh::{openssh_export, openssh_import};
use crate::r_crypt::io::{wasm_read, wasm_write};
use crate::r_crypt::WASM_MEMORY_BUFFER_SIZE;

// Define js functions we could use here
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn wasm_new_buffer() -> *const u8 {
    let shared_buffer: [u8; WASM_MEMORY_BUFFER_SIZE] = [0; WASM_MEMORY_BUFFER_SIZE];
    let shared_ptr: *const u8 = shared_buffer.as_ptr();
    return shared_ptr;
}

#[wasm_bindgen]
pub fn wasm_clean_buffer(wasm_buffer: *mut u8, len: usize) -> () {
    let patch: Vec<u8> = vec![0; std::cmp::min(len, WASM_MEMORY_BUFFER_SIZE)];

    unsafe {
        js_sys::Uint8Array::from(patch.as_slice()).raw_copy_to_ptr(wasm_buffer);
    }
}

#[wasm_bindgen]
pub fn get_WASM_MEMORY_BUFFER_SIZE() -> usize {
    return WASM_MEMORY_BUFFER_SIZE;
}


/**** Client: Generate ****/
// Assuming strings comes from https://docs.rs/ssh-key/latest/ssh_key/enum.Algorithm.html#method.new
// front
#[wasm_bindgen]
pub fn wasm_generate_keypair_ed25519(wasm_buffer: *mut u8) -> () {
    wasm_generate_keypair_files(wasm_buffer, ssh_key::Algorithm::new("ssh-ed25519").unwrap());
}

// front
#[wasm_bindgen]
pub fn wasm_generate_keypair_ecdsa(wasm_buffer: *mut u8) -> () {
    wasm_generate_keypair_files(
        wasm_buffer,
        ssh_key::Algorithm::new("ecdsa-sha2-nistp256").unwrap(),
    );
}

// front
#[wasm_bindgen]
pub fn wasm_generate_keypair_rsa(wasm_buffer: *mut u8) -> () {
    wasm_generate_keypair_files(wasm_buffer, ssh_key::Algorithm::new("ssh-rsa").unwrap());
}

// back
pub fn wasm_generate_keypair_files(wasm_buffer: *mut u8, algo: ssh_key::Algorithm) -> () {
    let mut offset: usize = 0;

    let passphrase: Vec<u8> = wasm_read(wasm_buffer, &mut offset);
    let (public_keyfile, private_keyfile): (Vec<u8>, Vec<u8>) =
        match generate_and_format(algo, passphrase) {
            None => {
                return;
            }
            Some((public_keyfile, private_keyfile)) => (public_keyfile, private_keyfile),
        };

    let public_length: usize = public_keyfile.len();
    let private_length: usize = private_keyfile.len();

    // if WASM_MEMORY_BUFFER_SIZE is > 65535, then you need to encode length in more than 2 bytes
    if ((public_length + private_length + 4) as usize) < WASM_MEMORY_BUFFER_SIZE {
        offset = 0;
        wasm_write(wasm_buffer, &mut offset, public_keyfile);
        wasm_write(wasm_buffer, &mut offset, private_keyfile);
    } else {
        unsafe {
            *wasm_buffer.add(0) = 255;
            *wasm_buffer.add(1) = 255;
        }
    }
}

// back back
//https://docs.rs/ssh-key/latest/ssh_key/public/struct.PublicKey.html#method.to_openssh
pub fn generate_and_format(algo: Algorithm, passphrase: Vec<u8>) -> Option<(Vec<u8>, Vec<u8>)> {
    // Generate a keypair
    let mut keypair: KeyPair = KeyPair::new(algo);
    keypair.generate();

    // Getting file content
    match openssh_export(keypair, passphrase, LineEnding::LF) {
        None => None,
        Some((public_key, private_key)) => Some((public_key, private_key)),
    }
}
/**** Client: Generate ****/

/**** Client: Sign ****/
//data to sign then private key + passphrase in openssh format
// front
#[wasm_bindgen]
pub fn wasm_sign(wasm_buffer: *mut u8) -> () {
    let mut offset: usize = 0;

    let data: Vec<u8> = wasm_read(wasm_buffer, &mut offset);
    //data may be empty, but we deny it
    if data.is_empty() {
        return;
    }

    let private_file: Vec<u8> = wasm_read(wasm_buffer, &mut offset);
    //if key isn't provided, unable to sign
    if private_file.is_empty() {
        return;
    }

    let passphrase: Vec<u8> = wasm_read(wasm_buffer, &mut offset);
    //empty passphrase is possible, not recommanded tho

    wasm_clean_buffer(wasm_buffer, offset);

    if let Some(signature) =
        import_and_sign(data, private_file, passphrase)
    {
        offset = 0;
        wasm_write(wasm_buffer, &mut offset, signature);
    };
}

// back
pub fn import_and_sign(
    data: Vec<u8>,
    private_file: Vec<u8>,
    passphrase: Vec<u8>,
) -> Option<Vec<u8>> {
    match openssh_import(Some(private_file), None, Some(passphrase)).0 {
        None => None,
        Some(keypair) => match keypair.sign(data) {
            None => None,
            Some(signature) => Some(signature),
        },
    }
}


// Some(keypair) => match keypair.sign(data) {
//     None => None,
//     Some(signature) => Some(signature),
// }

// Some(keypair) => {
//     let signature: Vec<u8> = match keypair.sign(data.clone()) {
//         None => Vec::new(),
//         Some(signature) => signature,
//     };
//     match keypair.verify(data,signature.clone()){
//         None => { crate::alert("Nop"); },
//         Some(_) => { crate::alert("Yeah"); },
//     };
//     Some(signature)
// }

/**** Client: Sign ****/ 

/**** Client: Verify ****/ 
// front
#[wasm_bindgen]
pub fn wasm_verify(wasm_buffer: *mut u8) -> bool {
    let mut offset: usize = 0;

    let data: Vec<u8> = wasm_read(wasm_buffer, &mut offset);
    //data may be empty, but we deny it
    if data.is_empty() {
        return false;
    }

    let signature: Vec<u8> = wasm_read(wasm_buffer, &mut offset);
    //if signature isn't provided, no need to verify it
    if signature.is_empty() {
        return false;
    }

    let public_file: Vec<u8> = wasm_read(wasm_buffer, &mut offset);
    //if signature isn't provided, impossible to verify
    if public_file.is_empty() {
        return false;
    }

    wasm_clean_buffer(wasm_buffer, offset);

    return match import_and_verify(data, signature, public_file)
    {
        None => false,
        Some(_) => true,
    };
}

// back
pub fn import_and_verify(
    data: Vec<u8>,
    signature: Vec<u8>,
    public_file: Vec<u8>,
) -> Option<()> {
    match openssh_import(None, Some(public_file), None).1 {
        None => { alert("Import failed"); None },
        Some(public_key) => {
            let keypair = KeyPair::new(public_key.algorithm());
            keypair.verify(data,signature,Some(public_key))
        }
    }
}

/**** Client: Verify ****/ 

// https://wasmbyexample.dev/examples/webassembly-linear-memory/webassembly-linear-memory.rust.en-us.html
//https://rustwasm.github.io/wasm-bindgen/reference/types/exported-rust-types.html

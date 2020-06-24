use std::ffi::CString;
use std::os::raw::c_char;

use libsodium_sys as sodium;
use zeroize::Zeroizing;

fn main() {
    unsafe {
        sodium::sodium_init();
    }

    let pwd = Zeroizing::new(String::from("password"));
    let mut buf = Zeroizing::new(vec![0 as c_char; sodium::crypto_pwhash_STRBYTES as usize]);

    unsafe {
        sodium::crypto_pwhash_str(
            buf.as_mut_ptr(),
            pwd.as_ptr() as *const c_char,
            pwd.len() as u64,
            sodium::crypto_pwhash_OPSLIMIT_INTERACTIVE as u64,
            sodium::crypto_pwhash_MEMLIMIT_INTERACTIVE as usize
        );
        println!("{}", CString::from_raw(buf.as_mut_ptr()).into_string().unwrap());
    }
}

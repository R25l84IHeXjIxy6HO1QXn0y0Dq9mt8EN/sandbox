#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::c_void;

include!("../bindings.rs");

const HASH_LEN: usize = 32;
const SALT_LEN: usize = 0;

fn main() {
    let mut hash: [u8; HASH_LEN] = [0; HASH_LEN];
    let mut salt: [u8; SALT_LEN] = [0; SALT_LEN];

    let t_cost: u32 = 2;
    let m_cost: u32 = 1 << 16;
    let parallelism: u32 = 1;

    let mut pwd = "password";

    unsafe {
        argon2i_hash_raw(
            t_cost, m_cost, parallelism,
            &mut pwd as *mut _ as *mut c_void, pwd.len() as u64,
            &mut salt as *mut _ as *mut c_void, salt.len() as u64,
            &mut hash as *mut _ as *mut c_void, hash.len() as u64);
    }
}

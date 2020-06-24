use std::thread::{spawn, sleep};
use std::time::Duration;
use std::fs::File;

use fs2::FileExt;

fn worker() {
    let f = File::open("foo.txt").unwrap();
    f.lock_shared().unwrap();
    println!("HELLO");
    sleep(Duration::from_secs(5));
}

fn main() {
    spawn(worker);
    sleep(Duration::from_secs(1));
    let f = File::open("foo.txt").unwrap();
    f.lock_exclusive().unwrap();
}

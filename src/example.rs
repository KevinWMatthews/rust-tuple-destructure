use std::sync::{Arc, Mutex, Condvar};

#[allow(unused)]
fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let (lock, cvar) = &*pair;
}

fn main() {
    stack();
    heap();
    copy_type();
    non_copy_type();
    ref_pattern();
    ref_pattern2();
    real_world_condvar();
}

#[allow(unused)]
fn stack() {
    let the_tuple = (1, 2, 3);
    let (x, y, z) = the_tuple;
}

use std::boxed::Box;

#[allow(unused)]
fn heap() {
    let the_tuple = (Box::new(1), Box::new(2));

    // individual values are moved out of tuple into variables
    let (box_x, box_y) = the_tuple;

    // Can't use the tuple again
    // let (a, b) = the_tuple;

    // Can dereference each Box
    let x = *box_x;
    let y = *box_y;
}

#[derive(Copy, Clone)]
struct CopyType {
    data: i32,
}

#[allow(unused)]
fn copy_type() {
    let the_tuple = (
        CopyType { data: 1 },
        CopyType { data: 2 },
    );

    let (ct1, ct2) = the_tuple;
    let x = ct1.data;
    let y = ct2.data;

    // Can use again - copied
    let (ct1, ct2) = the_tuple;
}

struct NonCopyType {
    data: i32,
}

#[allow(unused)]
fn non_copy_type() {
    let the_tuple = (
        NonCopyType { data: 1 },
        NonCopyType { data: 2 },
    );

    let (ct1, ct2) = the_tuple;
    let x = ct1.data;
    let y = ct2.data;

    // Can't use again - moved
    // let (ct1, ct2) = the_tuple;
}

#[allow(unused)]
fn ref_pattern() {
    let the_tuple = (Box::new(1), Box::new(2));

    // Borrows boxes from tuple
    let (ref box1, ref box2) = the_tuple;
    let x = **box1;
    let y = **box2;

    // Can still use the tuple
    let (box1, box2) = the_tuple;
}

#[allow(unused)]
fn ref_pattern2() {
    let the_tuple = (Box::new(1), Box::new(2));
    let (box1, box2) = &the_tuple;

    let x = **box1;
    let y = **box2;
}

#[allow(unused)]
fn real_world_condvar() {
    use std::sync::{Arc, Mutex, Condvar};

    let pair = Arc::new((Box::new(1), Box::new(2)));
    let (x, y) = &*pair;

    //TODO switch to Mutex, Condvar
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let (lock, cvar) = &*pair;
}

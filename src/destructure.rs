#![allow(unused)]

use std::boxed::Box;

fn main() {
    copy_type();
    non_copy_type();
}

fn copy_type() {
    let the_tuple = (1, 2, 3);

    // Elements are copied out of the tuple
    let (x, y, z) = the_tuple;

    // Can use the tuple again
    let (x, y, z) = the_tuple;
}

fn non_copy_type() {
    let the_tuple = (Box::new(1), Box::new(2));

    // Elements are moved out of the tuple
    let (box1, box2) = the_tuple;
    let x = *box1;
    let y = *box2;

    // Can't use the tuple again - the Boxes were moved
    // let (box1, box2) = the_tuple;
}

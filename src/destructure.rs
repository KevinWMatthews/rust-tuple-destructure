use std::boxed::Box;

fn main() {
    copy_type();
    non_copy_type();
}

#[allow(unused)]
fn copy_type() {
    let the_tuple = (1, 2, 3);
    let (x, y, z) = the_tuple;
}

#[allow(unused)]
fn non_copy_type() {
    // Non-Copy type
    let the_tuple = (Box::new(1), Box::new(2));

    let (box1, box2) = the_tuple;
    let x = *box1;
    let y = *box2;

    // Can't use again - the Boxes were moved out of the tuple.
    // let (box1, box2) = the_tuple;
}

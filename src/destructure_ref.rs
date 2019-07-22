fn main() {
    copy_type();
    non_copy_type();
}

#[allow(unused)]
fn copy_type() {
    let the_tuple = (1, 2, 3);

    // Borrow from the tuple - x, y, and z are references
    // This uses match ergonomics
    let (rx, ry, rz) = &the_tuple;

    // Old Rust requires something like:
    // let &(ref rx, ref ry, ref rz) = &the_tuple;

    // Can dereference and assign
    // Values are copied from the reference
    let x = *rx;
    let y = *ry;
    let z = *rz;
}

#[allow(unused)]
fn non_copy_type() {
    let the_tuple = (Box::new(1), Box::new(2));   

    // Borrow boxes from the tuple
    // Uses match ergonomics
    let (box1, box2) = &the_tuple;

    // Older Rust may require:
    let &(ref box1, ref box2) = &the_tuple;

    // `*box` dereferences the `&Box`, giving the actual Box.
    // `let` tries to move the Box into x, but this isn't allowed:
    // "cannot move out of borrowed content".
    // `box1` only borrowed the Box and can't give it away.
    // let x = *box1;

    // `*box1` dereferences &Box, giving a Box;
    // Apply `*` again to use the Box's Deref trait to get it's inner value.
    let x = **box1;
    let y = **box2;

    // Can still use the tuple
    let (box1, box2) = the_tuple;
}

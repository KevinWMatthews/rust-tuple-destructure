#![allow(unused)]

fn main() {
    copy_type();
    custom_copy_type();
    non_copy_type();
    custom_non_copy_type();
}

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

#[derive(Copy, Clone)]
struct CopyType {
    data: i32,
}

fn custom_copy_type() {
    // Ownership rules are similar to a basic Copy type, say, i32

    let the_tuple = (
        CopyType { data: 1 },
        CopyType { data: 2 },
    );

    // Elements are copied from the tuple
    let (ct1, ct2) = the_tuple;
    let x = ct1.data;
    let y = ct2.data;

    // Can use the tuple again
    let (ct1, ct2) = the_tuple;
}

fn non_copy_type() {
    let the_tuple = (Box::new(1), Box::new(2));   

    // Borrow boxes from the tuple
    // Uses match ergonomics
    let (box1, box2) = &the_tuple;

    // Older Rust may require:
    // let &(ref box1, ref box2) = &the_tuple;

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

// Do not derive Copy - type must be moved
struct NonCopyType {
    data: i32,
}

fn custom_non_copy_type() {
    // Ownership rules are similar to a Box

    let the_tuple = (
        NonCopyType { data: 1 },
        NonCopyType { data: 2 },
    );

    // Elements are moved from the tuple
    let (nct1, nct2) = the_tuple;
    let x = nct1.data;
    let y = nct2.data;

    // Can't use the again
    // let (nct1, nct2) = the_tuple;
}

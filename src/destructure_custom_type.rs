#![allow(unused)]

fn main() {
    copy_type();
    non_copy_type();
}

#[derive(Copy, Clone)]
struct CopyType {
    data: i32,
}

fn copy_type() {
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

// Do not derive Copy - type must be moved
struct NonCopyType {
    data: i32,
}

fn non_copy_type() {
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

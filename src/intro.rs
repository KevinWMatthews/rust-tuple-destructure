#[allow(unused)]
fn main() {
    // Explicit type
    let the_tuple: (i32, i32, i32) = (1, 2, 3);

    let x = the_tuple.0;
    let y = the_tuple.1;
    let z = the_tuple.2;

    // Inferred type
    let the_tuple = (1, 2, 3);
}

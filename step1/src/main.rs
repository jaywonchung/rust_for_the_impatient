/// Step 1 of the Rust for the Impatient tutorial.
///
/// Rust variables are immutable by default. Add the `mut` keyword in line 6.

fn main() {
    let x = 42;
    println!("x = {}", x);

    x = x + 1;
    println!("x = {}", x);
}

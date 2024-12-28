fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y += 1; // Modify x through y
    println!("x = {}", x); // Prints x = 6

    // This line will cause a compile-time error because z is an immutable reference
    // *z += 1; // This would cause a compile-time error because z is an immutable reference
}
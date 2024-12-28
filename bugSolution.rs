fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modify x through y
    println!("x = {}", x); // Prints x = 6
    
    //Correct way to use immutable reference
    let z = &x; 
    println!("x = {}", *z); //Prints x = 6
}
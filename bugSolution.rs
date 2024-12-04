fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y = 10; // Modify x through y
    println!("x = {}", x); // Output: x = 10
    
    //Now we demonstrate correct use of immutable reference 
    let z = &x; // z is an immutable reference to x
    println!("x = {}", *z); // Output: x = 10
} 
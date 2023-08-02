fn main() {
    println!("Showcasing Mutability");
    let mut x = -1.1;
    println!("The value of x is: {x}");
    x = 1.2;
    println!("The value of x after mutating it is: {x}");

    println!("Showcasing Shadowing");
    let y = 2;
    println!("The value of y is: {y}");
    let y = y * 2;
    println!("The value of y after doubling it is: {y}");

    println!("Showcasing Constants");
    const Z : i32 = 2;
    println!("The value of z is: {Z}");

}

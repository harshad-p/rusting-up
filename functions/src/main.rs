// Functions in Rust can have parameters or be parameterless. 
// Functions can also return values using expressions. 
// I will try to learn Rust functions 
// by trying to create a function that 
// prints a value with it's unit. 
// This should let me practice writing functions with parameters. 
// For practicing functions that return values, 
// I will try to call the same function, 
// but with value that is modified by a function that returns it. 
    
fn main() {
    println!("Hello from main function.");

    // Just using a sningle character unit for now. 
    let unit = 'm';

    let value = 5;
    print_value_with_unit(value, unit);

    let doubled_value = double_this(value);
    print_value_with_unit(doubled_value, unit);
}

// Function to demonstrate return values in the form of expressions. 
fn double_this(value: i32) -> i32{
    value * 2
}

// Function to demonstrate function parameters. 
fn print_value_with_unit(value: i32, unit: char) {
    println!("The value is: {value}{unit}.");
}
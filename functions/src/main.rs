fn main() {
    println!("Hello from main function.");
    let function_variable = 5;
    another_function(function_variable);
}

fn another_function(function_variable: i32) {
    println!("Hello from another function. I received this: {function_variable}");
}
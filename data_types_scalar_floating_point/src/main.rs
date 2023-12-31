fn main() {
    // It is necessary to add a decimal point, or else it won't compile. 
    // Although, the decimal point is not displayed on the console. 
    let floating_point_32 :f32 = -255.0;

    // By default, a fp number is initialized as f64. 
    // But when initializing by assigning a variable, it takes the type of the variable being assigned. 
    // Also, a 32-bit fp number cannot be assigned to 64-bit variable. 
    // let floating_point_64 :f64 = floating_point_32; // This does not work
    
    let floating_point_64 :f64 = -255255.0;

    println!("This is a signed 32-bit floating-point number: {floating_point_32}");
    println!("This is a signed 64-bit floating-point number: {floating_point_64}");
}

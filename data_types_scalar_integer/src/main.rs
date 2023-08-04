fn main() {
    let unsigned_8_int :u8 = 255;
    // This does not work
    // Cannot assign a signed int to unsigned int. 
    // Cannot even negate an int and assign it. 
    // let unsigned_8_int_Neg :u8 = -1 * unsigned_8_int; 
    println!("This is an unsigned 8-bit integer: {unsigned_8_int}");
}

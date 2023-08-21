fn main() {
    let unsigned_8_int :u8 = 255;
    println!("This is an unsigned 8-bit integer: {unsigned_8_int}");

    // This does not work
    // Cannot assign a signed int to unsigned int. 
    // Cannot even negate an int and assign it. 
    // let unsigned_8_int_Neg :u8 = -1 * unsigned_8_int; 

    // Assign a lower bit integer to a higher bit integer. 
    //let u_32_int: u32 = 32;
    //let u_64_int: u64 = u_32_int;
    //println!("This is an unsigned 64-bit integer assigned from 32-bit integer: {u_64_int}");
    
    // Assign a higher bit integer to a lower bit integer. 
    //let u_16_int: u16 = 16;
    //let u_8_int: u8 = u_16_int;    
    //println!("This is an unsigned 8-bit integer assigned from 16-bit integer: {u_8_int}");
}

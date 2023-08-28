fn main() {
    // Default Tuple initialization
    let t1 = (1, 2.2, 'z', true);
    // All tuples need to be deconstruction when using this pattern matching. 
    // If only the first 2 are needed, still all 4 need to be deconstructed. 
    // Also note how an int is initialized as a signed 32-bit integer in Rust, 
    // and Rust does not allow you to change the bitness after initialization. 
    // But here Rust does allow you to specify a non-default bitness. 
    // (I would have assumed that Rust would have initialized this integer as i32.)
    // Same for the floating-point number. 
    let (unsigned_8_int, floating_point_32, character, boolean) = t1;
    println!("Tuples: {unsigned_8_int}, {floating_point_32}, {character}, {boolean}");
}
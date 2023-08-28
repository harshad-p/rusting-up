fn main() {
    // Default Tuple initialization
    let t1 = (-1, 2.2, 'z', true);
    // All tuples need to be deconstruction when using this pattern matching. 
    // If only the first 2 are needed, still all 4 need to be deconstructed. 
    let (integer, floating_point, character, boolean) = t1;
    println!("Tuples: {integer}, {floating_point}, {character}, {boolean}");
    
    // Explicit type specification
    let t2: (u8, f32, char, bool) = (11, 22.22, 'a', true);
    let (unsigned_8_int, floating_point_32, character, boolean) = t2;
    println!("Tuples: {unsigned_8_int}, {floating_point_32}, {character}, {boolean}");

    let _t3 = (); // Cannot access value in empty tuple. 
}
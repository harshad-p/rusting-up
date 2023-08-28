fn main() {
    // Default Tuple initialization
    let t1 = (1, 2.2, 'z', true);
    // All tuples need to be deconstruction when using this pattern matching. 
    // If only the first 2 are needed, still all 4 need to be deconstructed. 
    let (unsigned_8_int, floating_point_32, character, boolean) = t1;
    println!("Tuples: {unsigned_8_int}, {floating_point_32}, {character}, {boolean}");
}
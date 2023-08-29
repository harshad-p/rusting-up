fn main() {
    // Default Array initialization
    let a1 = [-3, -2, -1, 0, 1, 2, 3];
    // I do not know yet how to access the array elements directly in the print stmt. 
    let first = a1[0];
    let third = a1[2];
    let seventh = a1[6];
    println!("Array elements: First: {first}, Third: {third}, Seventh: {seventh}");

    // Array initialization by specifying the type and length
    let a2: [i8; 7] = [-3, -2, -1, 0, 1, 2, 3];
    // I do not know yet how to access the array elements directly in the print stmt. 
    let first = a2[0];
    let third = a2[2];
    let seventh = a2[6];
    println!("Array elements: First: {first}, Third: {third}, Seventh: {seventh}");
}
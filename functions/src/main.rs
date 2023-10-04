fn main() {
    println!("Hello from main function.");
    let value = 5;
    let unit = 'm';
    print_value_with_unit(value, unit);
}

fn print_value_with_unit(value: i32, unit: char) {
    println!("The value is: {value}{unit}.");
}
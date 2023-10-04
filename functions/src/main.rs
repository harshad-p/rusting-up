fn main() {
    println!("Hello from main function.");
    
    let unit = 'm';

    let value = 5;
    print_value_with_unit(value, unit);

    let doubled_value = double_this(value);
    print_value_with_unit(doubled_value, unit);
}

fn double_this(value: i32) -> i32{
    value * 2
}

fn print_value_with_unit(value: i32, unit: char) {
    println!("The value is: {value}{unit}.");
}
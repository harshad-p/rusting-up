fn main() {
    let t = true;
    let t1 = t;
    let t2: bool = t1;
    let f = false;
    // I do not know how to do this casting yet. 
    //let f_int: i32 = (i32)f;
    //let f1: bool = 0;
    
    println!("This is a boolean variable: {t}");
    println!("This is a boolean variable: {t1}");
    println!("This is a boolean variable: {t2}");
    println!("This is a boolean variable: {f}");
    //println!("This is an integer variable casted from boolean: {f_int}");
    //println!("This is a boolean variable casted from integer: {f_1}");
}

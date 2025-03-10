fn main() {
    println!("variable declaration");
    let x = "55";
    print!("the value of x: {}\n", x);
    // x = "44"; cannot mutate immutable variable
    // mutables
    println!("mutables declaration");
    let mut y = "45";
    println!("the value of y: {}", y);
    y = "75";
    println!("the value of y: {}", y);
    //we can not mutate a varibales type

    // shadowing
    let age = 45;
    println!("the integer variable for age is : {}", age);
    let age = "enter your age";
    println!("{age}");

    //unused variables
    let _year = 2025;
    println!("constant declared for test: {}s", THREE_HOURS_IN_SECONDS);

    //called external function
    types();
    num_ops();
}
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn types() {
    //rust is statically typed
    //define variables with they types
    let my_variable: u8 = 42;
    println!("a variable defined with anotated data type {}", my_variable);
    // or we could also use suffixed typed
    let my_variable = 42u8;
    println!("a variable defined with suffixed data type {}", my_variable);

    let my_num = 1_000u64; //this also another way to 1000
    let my_num = my_num * 45;
    println!("my num was written as 1_000 * 45, result: {}", my_num);

    let minu: i32 = -9; //signed integer
    let val = minu + 11;
    println!("{}", val);

    let t = true;
    let f: bool = false;
    println!("booleans in rust: {}, {}", t, f);

    //floating types
    let width = 35.60f64;
    println!("a floating point number: {}", width);

    //character type
    let my_name: char = 'T';
    println!("character type: {}", my_name);
}

fn num_ops() {
    //addition
    let add = 5 + 10;
    println!("addition operation: {}", add);

    //difference
    let diff = 20 - 10;
    println!("subtraction operation: {}", diff);

    //multiplication
    let mul = 10 * 2;
    println!("multiplcation operation: {}", mul);

    //division
    let div = 75 / 25;
    println!("division operation: {}", div);

    //remainder
    let modulo = 75 % 15;
    println!("remainer(operation): {}", modulo);
}

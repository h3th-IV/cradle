use std::collections::HashMap;
use std::collections::VecDeque;


fn main() {
    println!("Hello, world!");
    let greeting_mesage = String::from("Hello World");
    let hello = _first_word(&greeting_mesage);
    println!("{hello}");
}

fn _primitive_types() {
    //all types discusssed here are of known size and stored on the stack
    // rust is statically typed
    // define variables with their types
    let my_variable: u8 = 42;
    println!(
        "a variable defined with annotated data type {}",
        my_variable
    );
    // or we could also use suffixed type
    let my_variable = 42u8;
    println!("a variable defined with suffixed data type {}", my_variable);

    let my_num = 1_000u64; // this is another way to write 1000
    let my_num = my_num * 45;
    println!("my num was written as 1_000 * 45, result: {}", my_num);

    let minu: i32 = -9; // signed integer
    let val = minu + 11;
    println!("{}", val);

    let t = true;
    let f: bool = false;
    println!("booleans in rust: {}, {}", t, f);

    // floating types
    let width = 35.60f64;
    println!("a floating point number: {}", width);

    // character type
    let my_rune: char = 'T'; // like rune in Go
    println!("character type: {}", my_rune as u32);

    // isize and usize are platform dependent pointer types (size)
    let _i: isize = 67;
    println!("the type of i: {}", std::any::type_name::<isize>());
}

fn _string_concatenation() {
    // concatenation of strings
    let first_name = "Thread";
    let second_name = "Miller";
    // we need to convert first_name to a string(own it as we cannot mutate &str type which is a reference)
    let name = first_name.to_string() + second_name;
    println!("Name: {}", name);
}

fn _ownership_and_borrowing() {
    // ownership and borrowing
    let s1 = String::from("One");
    println!("{}", s1);
    let s2 = s1;
    println!("{}", s2);
    // println!("{}", s1); //after here s1 is not available, because the ownership has been moved to s2

    // borrowing 2 types mutable and not mutable
    // immutable borrowing, s4 cannot mutate s3
    let s3 = String::from("Hello");
    let s4 = &s3;
    println!("s3: {} \ns4: {}", s3, s4);

    // mutable borrowing
    let mut s5 = String::from("Five");
    let s6 = &mut s5;
    s6.push_str("+1");
    // println!("s5: {}", s5); //s5 is immutably borrowed here, when it is already mutably borrowed
    println!("s6: {}", s6);
    println!("s5: {}", s5);

    let tup: (u32, i32, f64) = (10, 20, 45.5);
    println!("{}", tup.0);
    println!("{:?}", tup);
    // println!("{}", tup); // this would not work because std::fmt::Display is not implemented for a tuple type

    //this topic is still being studied
}

fn _compound_types() {
    // compound types
    /*
        tuples: element of different types, fixed size
        arrays: element of the same type, fixed size
    */
    let coordinate: (u32, f64, char) = (300, 60.007, 'T');
    // to get the individual values of a tuple we destructure it
    let (_x, y, _z) = coordinate;
    println!("the value of the y coordinate is {}", y);

    // x and y have underscores because they are unused, to avoid panic

    // also access tuple element by using dot notation with the index
    let ele = coordinate.0;
    println!("element access through dot notation of tuple: {}", ele);

    // array types
    let my_arr: [i32; 5] = [5, 4, 3, 2, 1];
    println!("my_arr element 0: {}", my_arr[0]);
}

// constants are generally immutable and can be defined in any scope
const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

/*
    learning functions
    functions are defined...
    fn func_name(parameter(s): type){} functions withour return values

    fn func_name(param: type) -> type_of_return_value {}
*/
fn _add_num(num_1: i32, num_2: i32) {
    let sum = num_1 + num_2;
    println!("{num_1} + {num_2} = {sum}");
}

//expressions(returns a value) and statements(performs actions, no value retunred)
fn _test() {
    let _c_v: &str = "Threader"; //this is a statement
    println!("{_c_v}");
    let x = {
        let y = 5;
        y + 1 //semicolon is not added to the end of (this line)an expression(returns value), if semeicolon is added it becomes a statement
              //everything in this block(curly braces) is an expression, it returns the value y
    }; //the outer let is a statement, it binds toa value and ends with semi colon
    println!("the value of x is {x}");
}

//we can return early with the use of return keyword and a value, or just use the expression in thefunction block as it's return value

// func without return statement(explicit)
fn _func_without_return() -> i32 {
    50
} //this function would return 50

// func with return statement(explicit)
fn _func_with_return() -> i32 {
    return 50;
}

//conditions
fn _if_esle() {
    let my_age = 20;
    if my_age < 18 {
        // the condition here(which should be an expression) must result to a bool type
        println!("you are not eligible to drive");
    } else {
        println!("pls proceed to get your license");
    }

    //if we try to pass an expression that doesnt result to a bool type as the condition to an if statement
    // if my_age { //this would return an error
    //     println!("you are {my_age} years old");
    // }

    // use of else if for multiple condition statement
    if my_age < 18 {
        println!("minors are not eligible to drive");
    } else if my_age > 70 {
        println!("senior citizens are not eligible to drive")
    } else {
        println!("pls proceed to get your license")
    }

    //if statement is an expression, so it returns value, therefore it can bind to a variable using the let keyword
    let dur = 48;
    let fiv = dur / 24;
    let duration = if dur % 24 == 0 {
        format!("{fiv} day(s)") //used the format macro here, creates string interpolation
    } else {
        "just some days".to_string()
    };
    println!("{duration}");
}

fn _loops() {
    //loop; kinda inifite loop
    let mut counter = 0;
    loop {
        counter += 1;
        println!("{counter}");
        if counter == 10 {
            break;
        }
    }

    println!("");
    //loop labelling: loops can be labelled to attach break or continue to them directly
    println!("loop labelling");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                println!("breaking inner loop");
                break;
            }
            if count == 2 {
                println!("breaking counting up loop");
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn _while_loops() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF");
}

fn _for_loop() {
    let arr: [i32; 5] = [10, 20, 30, 40, 50];

    for element in arr {
        println!("element: {element}");
    }
}

fn _clecius_fahrenheit(temp: i64) -> i64 {
    let ret_temp: i64 = temp * 9 / 5 + 32;
    return ret_temp;
}

fn _fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

fn _christ_mas() {
    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Golden Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];
    let day_arr = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    for element in 0..12 {
        println!(
            "on the {} day of christmas my true love sent to me {}",
            day_arr[element], gifts[element]
        );
    }
}

fn _ownrsheep() {
    {
        let mut my_name = String::from("Thread"); //momory is alloc on the heap
        my_name.push_str(" Miller");
        println!("{my_name}");
    } //outside the scope, it is dealloc from the heap (drop)
      // println!("{my_name}");
      //multiple data can interact with the same value in rust
    let x = 5;
    let _y = x;
    //since the two variables are point to the data 5 which is an integer, the size is known(fixed) at run time, these two variables are stored in stack.

    //when it comes to types that do not have fixed size, that are stored on the heap, we can not assign varibales to each other that way...
    let my_name = String::from("Thread Miller"); // this is stored on the heap, then a pointer to that memory allocated is stored on the stack;

    let name = my_name; //when we do something like this, the two varibales will be point to the same memory location on the heap,this means both variables are capable of deallocing(drop), the same memory location which becomes a bug

    //that is why the ownership of name is moved to my_name, so my_name becomes invalid
    println!("{name}");
    // println!("{my_name}");

    //use clone to copy both stack and heap data of variables
    let s1 = String::from("Threader");
    let s2 = s1.clone(); //the data was deeply copied here
    println!("s1: {s1}\ns2: {s2}");
}

//so i was looking collections types that are heap allocated, the size is determined at runtime
fn _var_size_collections() {
    //Vec<T> is a dynamically sized collection that can grow and shrink at runtime. It’s the most common way to handle collections with unknown or changing sizes.
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    //if you need a dynamically sized collection but want to allocate it on the heap and treat it like a slice, you can use Box<[T]>. This is a heap-allocated array (a fixed-size slice) where the size is determined at runtime, but once created, the size is fixed.
    let data: Box<[i32]> = (0..5).collect::<Vec<i32>>().into_boxed_slice();
    println!("{:?}", data); //output: [0, 1, 2, 3, 4]

    //if you need a collection where the size (number of elements) varies at runtime and the elements are key-value pairs, you can use a HashMap or a BTreeMap. These are also dynamically sized, and their length grows or shrinks depending on the number of elements they contain.
    let mut map = HashMap::new();
    map.insert("key1", 10);
    map.insert("key2", 20);
    println!("{:?}", map); //output: {"key1": 10, "key2": 20}

    //if you need a queue-like structure with dynamic sizing, a VecDeque can be used. It's similar to a Vec, but optimized for adding and removing elements from both ends.
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_back(1);
    deque.push_front(2);

    println!("{:?}", deque); // Output: [2, 1]
}

fn _ownr_functions() {
    let name = String::from("Thread Miller"); //comes into scope, assigned space in the heap memory
    _take_ownr(name); //ownership is moved(for String type) to the function.
                      // println!("{name}"); this is will return error as value has been moved

    let num: u32 = 50; //num comes into scope and is assigned space in the stack memory
    _make_copy(num); //ownership is not moved as the integer type has copy trait
    println!("{num}"); //this will work cause the value was only copied.
}

fn _take_ownr(text: String) {
    //the string comes into scope(moved into the function)
    println!("String type moved here: {text}");
    //the string is moved out of scope and memory is released.
}

fn _make_copy(int: u32) {
    //the integer comes into scope here
    println!("Integer type element was copied here: {int}");
    //the integer goes out of scope here
}

fn _return_ownr() {
    //returning values can also transfer ownership
    let s1 = _gives_ownr(); // gives_ownr moves its return value into s1
    println!("{s1}");

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = _takes_give_ownr(s2); // s2 is moved into takes_give_ownr, which also moves its return value into s3
    println!("{s3}");
}

fn _gives_ownr() -> String {
    let name: String = String::from("Hrekkf");
    return name;
}

fn _takes_give_ownr(a_string: String) -> String {
    return a_string;
}

//because of ownership, a value we might still have use goes out of existence, say when pass ot a function, i find this very annoying...very very annoying.
//one way of doing this, is to return a tuple containing the original value when using functions

fn _calculate_lenght(text: String) -> (String, usize) {
    let lenght = text.len();
    return (text, lenght);
}

/*
But this is too much ceremony and a lot of work for a concept that should be common.
Luckily for us, Rust has a feature for using a value without transferring ownership, called
references.
*/

fn _use_reference() {
    //A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that sreference.

    let x = 78;
    let _y = &x; //a reference of x is here passed as the value to y
    let name = String::from("Thread Miller");
    _ref_calculate_lenght(&name); //the refrecne of the value name is passed to the function so that ownership is not moved and we ares still able to use the value
    println!("{name}"); //no error here isnce ownership was not moved.

    //we canot modify reference to a variable....but we can a mutable reference.
}

fn _ref_calculate_lenght(text: &String) -> usize {
    text.len()
}

fn _mut_reference() {
    //mutable references
    let mut s = String::from("hello");
    _change(&mut s);
    println!("original value changed after modification: {s}");
}

fn _change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("modified in the function: {some_string}");
}

//we can not have more than one mutable reference of a value
fn _only_one_mut_ref() {
    let mut s = String::from("Hello");
    let x = &mut s;
    println!("{x}");
    let y = &mut s;
    println!("{y}");
    //this will cause race conditions as two vars are trying to mutate the same mem location concurrently(data race)
    // println!("{x}, {y}");
    //or use seperate scope
    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let _r2 = &mut s;
}

//we can not have a mutable and immutable reference together
fn _no_mut_and_imut_ref() {
    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM:cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("{}, {}, and {}", r1, r2, r3); //this will return error

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    //variables r1 and r2 will not be used after this point
    let r3 = &mut s; //no problem
    println!("{r3}");
}

fn _string_slice() {
    let name = String::from("Thread Miller");
    let first_name = &name[0..6]; // or use [..6]
    let last_name = &name[7..name.len()]; //or use [7..]

    println!("my First Name is {first_name}, my Last Name is {last_name}, hence {name}");

    let user_name = "Thread Miller";
    let f_name = &user_name[..6];
    println!("{f_name}");
}

fn _first_word(text: &str) -> &str {
    //convert text to an array of bytes.
    let string_byte = text.as_bytes();
    //enumerate returns a tuple containg index and item(value), hence we destructure it
    for (i, &item) in string_byte.iter().enumerate() {
        //check if item equals the binary representation of space
        if item == b' ' {
            return &text[..i]; //return all ietm from the begining to current position
        }
    }
    &text[..] //return all item if no space is found.
}

//STRUCT
struct User {
    email: String,
    last_name: String,
    password: String,
    active: bool
}
//create an instance of struct
fn build_user(email: String, name: String, password: String) -> User{
    User { email, last_name: name, password, active: true }
}
fn structs(){
    //use build function to instantiate a new struct
    let name: String = String::from("Thread");
    let email: String = String::from("threadmiller@clother.com");
    let user1: User = build_user(email, name, String::from("password"));

    //we could also use an existing struct to create a new struct
    let email2: String = String::from("woolermiller@clother.com");
    let user2 = User{
        email: email2,
        ..user1
    };
    println!("user1 name cannot be used anymore here since it's value has been moved to user2. name, ");
    println!("user1 email is {}", user1.email);
}

//tuple struct
fn tuple_struct(){
    struct Color(i32, i32, i32); //tuple struct dont have named fields just types.

    let black: Color = Color(0, 0, 0);
}

//unit-likje structs
fn unit_like(){
    struct AlwaysEqual; //dont have fields just name, can be used to implement traits on types.
}


//a derived type like structs cannot be debugged with the println!() macro
//we add the attribute #[derive(Debug)] before struct definition

#[derive(Debug)]
struct Human {
    race: String,
    name: String,
    age: u8,
    height: f64,
}

fn print_derived_type(){
    let race: String = String::from("African");
    let name: String = String::from("Khem");
    let age: u8 = 40;
    let height: f64 = 5.5;
    let new_human: Human = Human { race, name, age, height};
    println!("A human struct representation printed, {new_human:#?}");

    //use dbg! macro to print to stderr
    dbg!(&new_human);
}

//struct and methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//methods defined in the impl block
impl Rectangle {
    //calc the area of rectangle
    fn area(&self) -> u32{
        self.width * self.height
    }

    //return the width
    fn width(&self) -> u32{
        self.width
    }

    //associated functions; this does not take in &self as their first parameter, therefore, they are not methods, they can be used as constructors, they are called with the namespace syntax "::" e.g Rectangle::square(3)
    
    //square is an associated function, that creates a rectangle with equal sides
    fn square(size: u32) -> Self {
        Self { width:size, height: size }
    }
}

fn use_methods(){
    let rect: Rectangle = Rectangle { width: 30, height: 40 };
    //debug the area calculation
    dbg!(rect.area());

    //print the width to standard output
    println!("the width of the rectangle {rect:#?} is {}", rect.width());

    let new_square: Rectangle = Rectangle::square(40);
}

//enum are types that can have one specific variants, e.g a shape can be a circle, triangle e.t.c
enum Shape {
    Triangle,
    Circle,
    Rectangle,
    Square,
}

//variants are namedspace from their enum type

//this will match which shape which is passed to it
fn use_enum(shape: Shape) -> String{
    match shape {
        Shape::Triangle => String::from("Shape Triangle"),
        Shape::Circle => String::from("Shape Circle"),
        Shape::Rectangle => String::from("Shape Rectangle"),
        Shape::Square => String::from("Shape Square"),
    }
}

fn get_shape(){
    let circle: Shape = Shape::Circle;
    println!("{}", use_enum(circle)); //this will print Shape Circle
}


//use of predefined enums; Option and Result
/*
    Option is used to handle situations where a value may or may not be present.
    enums Option<T>{
        Some<T>,
        None,
    }

    - Some(T) represents a value of type T that is present.
    - None represents the absence of a value.

    Result is used to handle situations where a computation may succeed or fail.
    enums Result<T, E>{
        Ok(T),
        Err(E),
    }

    - Ok(T) represents a successful outcome with a value of type T.
;   - Err(E) represents an error with a value of type E.
 */

//usage

//option usage
fn match_name_by_id(id: i32) -> Option<String>{
    match id {
        1 => Some("Thread Miller".to_string()),
        2 => Some("Wooler Miller".to_string()),
        3 => Some("Seamer Miller".to_string()),
        _ => None, //any other value will default to None
    }
}


// result usage
fn do_div(num: i32, denom: i32) -> Result<i32, &'static str> {
    if denom == 0{
        Err("cannot do division by 0")
    } else{
        Ok(num/denom)
    }
}

fn match_div(){
    let num: i32 = 45;
    let denom: i32 = 15;
    match do_div(num, denom) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("{}", err)
    }
    //result and err captures the result returned for each  of the variants
}

enum IpAddr {
    v4,
    v6
}

fn match_with_match(ipaddr: &IpAddr) {
    match ipaddr {
        IpAddr::v4 => println!("ip address version 4"),
        _ => ()
    }
}

fn match_with_if_let(ipaddr: &IpAddr) {
    if let IpAddr::v4 = ipaddr{
        println!("ip address version 4")
    }
}

fn check_fuel(){}

pub mod drive_car{
    pub fn turn(direction: &str){
        let tire_direction:Tires = engage_tires(direction);
        println!("front tires will move like so, {} while rear tires {}", tire_direction.front, tire_direction.rear)
    }

    fn engage_tires(direction: &str) -> Tires{
        let tires: Tires = Tires { front: String::from(direction), rear: String::from("drag") };
        tires
    }

    pub fn drift_tires(direction: &str) -> Tires{
        let tires: Tires = Tires { front: String::from(direction), rear: String::from("drag") };
        tires
    }

    pub fn reverse(){
        super::check_fuel(); //to use item in the parent module we use the super keyword like ../check_fuel
    }

    pub fn drive(){
        super::check_fuel();
    }

    pub struct Tires{
        pub front: String,
        rear: String
    }
}

fn main_test(){
    drive_car::reverse(); //reverse is a public fn in the drive
    drive_car::drive();

    let drift_dir: drive_car::Tires = drive_car::drift_tires("left_corner");
    println!("a sharp left ahead, drift: {}", drift_dir.front);
    // println!("will the rear tires {}", drift_dir.rear); //error here since rear field is private
}
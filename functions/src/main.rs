// 09/01 Complex types: Chang-Yong Song

fn main() {

    // The Functions are permeated at the RUST.
    // The keyword "fn" makes the function declared.
    // Rust's Rule: All the name of functions and values are declared with the "snake name"
    // snake name: Use lower and replace blank by '_'

    println!("Hello, world!");

    func_foo();

    func_foo_2(5); // Parameter: 5
    func_foo_3(5, 4); // Parameters: 5, 4

    // Experssion: Return the value.
    // Synax: Sequence of the Intruction, and do not return the value.

    // This is syntax.
    let z = 6;    

    // However, Rust does not permit this. x = y = z
    // let x = (let y= 6); -> Error is occured.

    // Below example return '4' -> Bounded at 'y'
    let z = {
        let x = 3;
        x + 1
    };

    let number = 3;

    // If expression
    if number < 5 {
        println!("Condition was TRUE");
    } else {
        println!("Condition was FALSE");
    }

    // NOTE: In this if ~ else, the condition MUST be "bool"
    // if number {
    //     println!("Condition was TRUE");
    // } else {
    //     println!("Condition was FALSE");
    // }

    // We can also use the if at the let syntax.
    // But, you MUST make the same type at these. -> Very Interesting..
    let condition = true;
    let number = if condition {
        5
    } else {
        6 // NOT SIX!
    };

    println!("The value of number is: {}", number);

    loop { // Infinity Loop
        println!("Again!");
    }

    // Conditional Loop
    let mut number = 3;
    while number != 0 {
        println!("{}", number);

        number = number - 1;
    }

    // For Loop
    let a = [1, 2, 3, 4, 5];
    for elements in a.iter() { // Use Iterator, and SAFE!
        println("the value is {}", element);
    }
}

// Function's declaration location is not fixed. We can locate this fucntion above the "main" function
fn func_foo() {
    println!("I am Another function!");
}

// By using argument at the function, we transfer the value via this. 
// In this example, we use x and the type of x is "i32".
// Programmer must declare the type of the parameters!! (IMPORTANT)
fn func_foo_2(x: i32) {
    println!("The Value of x is: {}", x);
}

 // We also can use multiple arguments
fn func_foo_3(x: i32, y: i32) {
    println!("The Value of x is: {}", x);
    println!("The Value of x is: {}", y);
}

// Absolutely Function :) Return type is "i32".
// This is same with let x = 5;
fn foo_five() -> i32 {
    5
}

// This function return x + 1
fn plus_one(x: i32) -> i32 {
    x + 1 // NOTE: WE DO NOT USE ':' at the RETURN VALUE! -> MISMATCHED TYPES ERROR
}
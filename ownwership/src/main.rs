// 09/08 Ownership: Chang-Yong Song

// Ownership is the important property at the rust programming.
// We can use this intuitively, but this affects the rest of language.
// All program (or process) have to find how they use the memory resources
// Some languages have their own Garbage Collector(GC) -> We have to use "free" function (C: free, C++ : delete)
// Rust use another appraoches by ownership at the compile time.

// IMPORTANT: We have to consider that the value exists at the stack or heap!
// For Rust, whether the value exists at the stack or heap affects how it works.
// STACK: LIFO (Last In First Out), Fast(Stack Pop, Push at the Top), All data's size are fixed
// HEAP: FIFO(First In First Out), Find the available location to allocate value -> SLOW (Follow the Pointer)

// Rules
// 1. Each of the value at Rust has "owner" variable
// 2. There can be only ONE owner at a time
// 3. If the owner ruled out the scope, dropped.


fn main() {
    // Scope: The range to indicate whether the item is valid or not.
    // At this line 22, the variable "s" is not valid because it was not declared.
    let s = "hello"; // At this line 23, the variable "s" is valid.
    // We can do anything by using the variable "s". 

    let s1 = String::from("hello");
    // 1. String variable is immutable.
    // 2. However, we could not know when we make the code. -> If we want to save the value when the user inputs the string?
    // Thus, the string type is allocated at the heap, and we save unknown amount of the values at compile time.
    // By using "from" function, we can use string variables.
    // At that time, :: means namespace operator.
    // Therefore we could NOT change the variable "s1".

    // However, we can chage the value s2! The difference in these 2 examples is how they access the memory.
    let mut s2 = String::from("hello");
    s2.push_str(", world!");
    println!("{}", s2);

    // String Literal (s1): we could know the content at the variable at the COMPILE TIME!
    // Very Fast, efficient -> However, It is assumed that the string variable does NOT CHANGE!
    // Therefore, If we want to change this variables at run-time, we have to consider below.
    // 1. At the runtime, memories are allocated from the OS.
    // 2. After the use of String, we have to free this variable.

    // [1] : We do it ourselves.
    // [2] : GC (At Rust, if the varaible dropped at the scope, automatically call GC)

    // Move : The interaction between variables and data
    let x = 5;
    let y = x;
    // It is common example at programming language.

    let s1 = String::from("HELLO?");
    let s3 = s1; // s1 is NOT VALID since this line. 
    println!("{}, world!", s1);
    //ERROR!
    // If these 2 variables (s1, s3) are dropped at the scope, they free exact same memory. 
    // This problem so called "Double Free" Error -> Memory Corruption can be occured.
    // At Rust, we consider that s1 is not valid anymore at this scope by using "MOVE".
    // Rust prevents that s1 access to the value when they dropped.
    // However, s3 stil points the value "HELLO" 

    // If we want to copy the value (Deep Copy), we can use "clone()" method.
    let s1 = String::from("hello?");
    let s2 = s1.clone();

    // The data at the stack can be copied like this.
    let a = 5;
    let b = a;

    println!("x = {}, y = {}", x, y);
    // This can be done successfully because it does not use heap allocation.

    // Ownership and Function
    let c = String::from("hello?"); // c come inside of the scope.

    takes_ownership(c); // c is NOT VALID since this line.

    let d = 5;

    makes_copy(d); // although d comes inside of the function, because i32 can copy, we can still use varaible d
    // Copy: Integer, bool, float, set of these(Copy traits)

    let e = gives_ownership(); // function "gives_ownership" moves the return value to "e"

    let f = String::from("hello?"); // f comes in the scope.

    let g = takes_and_gives_back(f); // f moves out to takes_and_gives_back, and this function also moves return value to "f".
}
// Scope end, the variable "s" is not valid anymore.
// In the scope, the time when the variable "s" is appeared, that is valid.


fn takes_ownership(some_string: String) { //some_string comes in the function
    println!("{}", some_string);
} // some_string is dropped.

fn makes_copy(some_integer: i32) { // some_interger comes in the function
    println("{}", some_integer);
} // Nothing happens!

fn gives_ownership() -> String { // givew_ownership function moves return value from function to caller.
    let some_string = String::from("hello?"); // some_string comes in the scope.

    some_string // Return, and move to caller function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes in the function.
    a_string // Return, and move to caller function.
}
// 09/08 Reference & Borrowing: Chang-Yong Song

// If we want that although functions can use variables, do not give ownership? -> Reference!
// Instead of transfering the ownership, we can use reference as an arguments at the function
fn main() {
    let s1 = String::from("hello?");

    // We refer to the value s1, but we do not give ownership to function.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// Use reference &String, & means "Reference"
fn calculate_length(s: &String) -> usize { // S is reference of the String.
    s.len() 
} 
// Although s is dropped from the function, because s does not have ownership, nothing happens.
// This is called "Borrowing" Therefore, we could NOT change the value like const& :(

fn chage(some_string: &mut String) {
    some_string.push_str(", world!");
}
// However, if we use "Mutable Reference", we could change the value
// IMPORANT: There can be only ONE Mutable Reference at the ONE scope.
// let r1 = &mut s;
// let r2 = &mut s; -> This is ERROR! (Data Race)
// Thus, we have to use "{}".

fn dangle() -> &String {
    let s = String::from("Hello?");

    &s
}
// Dangling Reference
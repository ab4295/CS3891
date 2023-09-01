// 09/01 Complex types: Chang-Yong Song

fn main() {
    // Array: Set the serveral values. Likewise tuples, array must have the several same value sequences
    // Rust's array always have the "fixed" length -> If array declare at once, it does not shrink or enlarge.
    // At Rust, we seperate value by using ','
    let a = [1, 2, 3, 4, 5];

    // Array is useful if you want to store the values at not the heap but stack or If programmer is convined that
    // The array's size is always fixed.
    // However, we can also use the vector which is similar with the C++. Vector is pseudo - set that the std library serviecs this.
    // Vector can decrease or increase the size.

    // The example of chossing case: Array -> month is not added or erased because their elements are fixed.
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", 
        "Sep", "Oct", "Nov", "Dec"];

    // Array is allocated at the stack for single memory We can access the index like this.
    let first = a[0];
    let second = a[1];

    println!("First: {}", first);
    println!("First: {}", second);

    println!("This Month: {}", months[8]);
}

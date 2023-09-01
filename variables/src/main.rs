
fn main() {
     // 1. Variables
     // Similar with the "constant" at the C/C++
     // However, constant is inmutable itself! not basic setting
     // Also, constant is not allowed to use mut
     // Finally, constant would be set only via constant expression
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // 2. Shadowing
    // First, bind the value 5 at the variable a
    let a = 5;
    // By using repeated let, we can shadow the value and can update the value
    let a = a + 1;
    let a = a * 2;

    // Final value will be updated from 5 to 12
    println!("The value of a is: {}", a);

    // Discussion: mut vs shadow
    // If we use shadowing, although we update the value, variable is always "inmutable"
    // We also can use same variable, change the type and even use same name.

    // let spaces = "   ";
    // let spaces = spaces.len();

    // let mut spaces = "    ";
    // spaces = spaces.len();
}

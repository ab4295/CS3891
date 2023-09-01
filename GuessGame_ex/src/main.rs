// Crate: Package of the rust code. It is similar with the External lib
// By using "exrtern crate", we inform that the crate which depends on the external exists.
// If we want to use crate, we add this at the "dependencies" of "Cargo.toml"
extern crate rand; 


use std::io; // std lib for use input, output
use std::cmp::Ordering; // std lib for use Less, Greater, Equal
use rand::Rng; // Trait which several methods are declared

//Function Main
fn main() {
    println!("Guess the number!");

    // let: immutable, thread_rng: OS decides the seeds and return the random value.
    // At that time, by using gen_range, we can decide the range of the number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop{
        println!("Please input your guess.");

        // let mut: mutable, new(): return new string instance (blank)
        let mut guess = String::new();
    

        // read_line: when user input at the stdin, save the data at the string.
        // At that time, we need variable for store the string, and this would be "mutable"
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // read_line not only returns the string value, but also returns the "Result" type (GENERIC)
        // Result: enumerations, elements are called "variants". Result's variants are "Ok" and "Err"
        // Therefore, if the error is occuered at the read_line func, "Err" and Msg will be returned
        // Also, the program will be dead. If we do not call the "expect", compile can be done. However, the "warning" will be appeared
    
        // We have to change the type to use cmp function
        // Rust Property: Rust allows shadowing the value (If we want to change the type)
        // Parse: Parsing from string to number
        // If invalid value is inputed, ignore this input
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num, 
            Err(_) => continue,
        };

        // {}: Placeholder, like %d at the C
        println!("You guessed: {}", guess);

        // If ~ else at the C
        // By using cmp method and the parameter secret number, we can compare the value between guess & secret number
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
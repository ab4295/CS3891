fn main() {
    // Rust is fixed-type language.
    // This means that all the variable's type have to be set before compile

    // Calculations
    let _sum = 5 + 10;

    let _differenece = 95.5 - 4.3;

    let _product = 4 * 30;

    let _quotient = 56.7 / 32.2;

    let _remainder = 43 % 5;

    // Boolean
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // Character: '', String: ""
    let _c = 'z';
    let _z = 'Z';
    let _heart_eyed_cat = "😻";

    // Complex
    // Make the tuple by associating values
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    // Structural dismantling
    let (_x, _y, _z) = _tup;
    println!("The value of y is: {}", _y);

    
}

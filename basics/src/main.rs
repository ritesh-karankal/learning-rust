fn main() {
    // Variable and mutibility
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constant
    const MAX_POINTS: u32 = 100_000;
    println!("The constant is {}", MAX_POINTS);

    // Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    // let mut spaces = "  ";
    //spaces = spaces.len(); //error
    
    // Data types
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is {}", guess);
    
    // Scalar types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // Boolean type
    let t = true;
    let f: bool = false;

    // Character type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
   
    // Compound types
    // Tuple
    let tup : (i32, f64, u8) = (132, 4.5, 2);
    let (x, y, z) = tup;
    println!("The value of y is {}", y);

    let one_thirty_two = tup.0;
    println!("The value of one_thirty_two is {}", one_thirty_two);
    let four_point_five = tup.1;
    let two = tup.2;

    // Array
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September",
                  "October", "November", "December"];

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is {}", first);

}


fn main() {
    another_function(80, 50);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
        };

    println!("The value of y is {}", y);

    let x = six();

    println!("The value of x is {}", x);

    let x = plus_one(5);
    println!("The value of x is {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {} and y is {}", x, y);

    
}

fn six() -> i32 {
    6
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
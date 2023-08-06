#![allow(unused)]
enum MyOption<T> {
    None,
    Some(T),
}

fn main() {
    let some_number = MyOption::Some(7);
    let some_char = MyOption::Some('a');

    let absent_number: MyOption<i32> = MyOption::None;

    // In rust only same type integers can be added
    // i8 + i32 gives error
    let x: i8 = 6;
    let y: i8 = 4;

    // let z: MyOption<i8> = Some(5);
    // x + z cannot be added give err - no implementation for `i8 + MyOption<i8>`
    let m = MyOption::Some(9);

    // println!("The sum is {}", x + m);

    let number1 = MyOption::Some(10);
    let number2 = MyOption::Some(20);

    println!("The sum is {}", number1 + number2);
}
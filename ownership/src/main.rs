fn main() {
    // let s = "hello";

    let mut m = String::from("hello");
    m.push_str(", world");

    println!("{m}");


    let x = 5;
    let y = x;
    println!("{}, {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, {}", s1, s2);

    // Ownership and functions

    let s = String::from("hello");
    take_ownership(s);
    // println!("{}", s);

    let v = 5;
    makes_copy(v);
    // copies it so we can use it here
    println!("{}", v);

    // Returning value and scope
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_givesback(s2);
    println!("{s3}");

    let s1 = String::from("length");

    let (s2, len) = calc_len(s1);

    println!("The length of '{}' is {}.", s2, len);

}

fn take_ownership(str: String) {
    println!("{}", str);
}

fn makes_copy(the_integer: i32) {
    println!("{}", the_integer);
}

fn gives_ownership() -> String {

    let some_string =  String::from("Some String");

    some_string
}

fn takes_and_givesback(a_string: String) -> String {

    a_string
}


fn calc_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
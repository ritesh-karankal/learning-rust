fn main() {
    let s1 = String::from("length");
    // Reference
    let len = calc_length(&s1);

    println!("{}, {}", s1, len);


    // Mutable reference
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{s}");
    

    // More than one reference to same variable
    let mut s = String::from("More than one reference");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s; 

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);


    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);


    let reference_to_nothing = dangle();
}


fn calc_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

/* 
Let’s recap what we’ve discussed about references:

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.
*/
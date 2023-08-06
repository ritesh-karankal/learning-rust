enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater,
}

fn value_in_cents(coin: COin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater => 25,
    }
}

fn main() {
     
}

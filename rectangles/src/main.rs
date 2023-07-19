#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

fn main() {
    // Using varible regularly
    // let width = 30;
    // let height = 50;
    // println!("The area of the rectangle is {} square pixels.", area1(width, height))

    // Using Tuple
    // let rec1 = (30, 50);
    // println!("The area of the rectangle is {} square pixels.", area2(rec1))


    let my_rec = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area(&my_rec));

    // printing our rectangle struct
    println!("rect is {:?}", my_rec);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

// fn area1(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area2(dimension: (u32, u32)) -> u32 {
//     dimension.0 * dimension.1
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
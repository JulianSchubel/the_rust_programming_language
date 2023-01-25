#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let scale:u32 = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    
    println!("The area is {:#?} sqaure pixels.", rect);
    dbg!(&rect);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //In main, when we follow rect1.width with parentheses, Rust knows we mean the method width. 
    //When we don’t use parentheses, Rust knows we mean the field width
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //not a method since it doesn't take &self, just an associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect0 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect0.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


}

//borrow struct rather than take owersnhip with &
//clear and descriptive
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//values are not associated with eachother
fn area_plain(width: u32, height: u32) -> u32 {
    width * height
}

//unclear - which is width and which is height?
fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
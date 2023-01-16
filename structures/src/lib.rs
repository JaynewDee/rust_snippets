#![allow(dead_code)]

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

// A unit struct
pub struct Unit;

// A tuple struct
pub struct Pair(pub i32, pub f32);

// A struct with two fields
pub struct Point {
    pub x: f32,
    pub y: f32,
}

// Structs can be reused as fields of another struct
pub struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    pub top_left: Point,
    pub bottom_right: Point,
}

mod my {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

impl Rectangle {
    pub fn area(&self) -> f32 {
        let (x_one, y_one) = (self.top_left.x, self.top_left.y);
        let (x_two, y_two) = (self.bottom_right.x, self.bottom_right.y);
        println!("x_one: {}\ny_one: {}", x_one, y_one);
        println!("x_two: {}\ny_two: {}", x_two, y_two);
        0.0
    }
}

pub fn visibility() {
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox {
        contents: "public information",
    };

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `ClosedBox` has private fields
    //let closed_box = my::ClosedBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let _closed_box = my::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    //println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line
}

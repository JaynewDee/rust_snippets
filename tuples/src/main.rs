use std::fmt::{self, Display};

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
fn main() {
    println!("Hello, world!");

    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Extract values using tuple indexing `.` syntax:
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple fifth value: {}", long_tuple.4);

    // Tuples can have tuples as members:
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable with debug format,
    // if there are 12 or fewer elements:
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);

    println!("pair is {:?}", pair);

    println!("REVERSED pair is {:?}", reverse_pair(pair));

    // To create single-element tuples, a comma is required,
    // to distinguish from literals surrounded by parentheses:
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // Tuple destructuring:
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?},", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}

fn reverse_pair(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

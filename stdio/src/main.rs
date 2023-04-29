use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Enter a message: ");

    io::stdin()
        .read_line(&mut buffer)
        .expect("Read line shoulda done the buffer thing");

    println!("buffer is {}", buffer);

    let number: i32 = buffer.trim().parse().unwrap();

    println!("{}", number + number);
}

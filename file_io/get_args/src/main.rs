use std::env::args;

fn main() {
    for (idx, arg) in args().enumerate() {
        // println!("Index: {}\nArgument: {}", idx, arg);
    }

    println!("{}", nth_arg(2));
}

fn nth_arg(n: usize) -> String {
    args().nth(n).unwrap()
}

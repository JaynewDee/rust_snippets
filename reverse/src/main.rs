use std::env::args;

fn main() {
    let input: Vec<String> = args().collect();

    //

    if input.len() > 2 {
        panic!("Too many args !!!");
    }
    if input.len() == 2 {
        println!("{:?}", reverse(&input[1]));
        return;
    } else {
        println!("{}", reverse("Hello, world!"));
        return;
    }
}

fn reverse(input_str: &str) -> String {
    String::from(input_str).chars().rev().collect()
}

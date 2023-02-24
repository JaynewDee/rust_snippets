fn main() {
    // As one-liner ?

    let reversed = reverse("Hello, world!");

    println!("{}", reversed);

    // NICE
}

fn reverse(input_str: &str) -> String {
    String::from(input_str).chars().rev().collect()
}

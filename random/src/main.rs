use rand::prelude::*;

fn main() {
    let rand_num = random::<f64>();
    println!("Random number is: {}", rand_num * 1000 as f64);

    let rand_by_range = thread_rng().gen_range(1..=15);

    println!("Random number from range: {}", rand_by_range);
}

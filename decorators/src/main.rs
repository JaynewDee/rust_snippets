use std::time::{Duration, Instant};

fn timer<F, T>(mut f: F) -> impl FnMut(T) -> T
where
    F: FnMut(T) -> T,
{
    move |args| {
        let start = Instant::now();
        f(args);
        let elapsed = start.elapsed();
        println!("Elapsed time: {:?}", elapsed);
    }
}

fn cpu_spinner(x: i32) {
    let mut nums = [x; x];
    for i in 0..=x {
        nums[i] = x;
    }
    let mut sum = 0;
    for j in 0..nums.len() {
        sum += nums[j];
    }
}

fn main() {
    println!("Hello, world!");
}

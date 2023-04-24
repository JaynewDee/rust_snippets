fn main() {
    println!("Hello, world!");
}

struct Counter;

impl<F> Counter {
    fn new() -> (F, F)
    where
        F: Fn() -> (F, F),
    {
        let mut count: i16 = 0;

        let increment = || {
            count += 1;
        };
        let decrement = || {
            count -= 1;
        };

        (increment, decrement)
    }
}

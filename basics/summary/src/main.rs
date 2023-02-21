fn main() {
    println!("Hello, world!");
}

fn tuple() {
    let pair = ("@pple", 13);
    pair.0; // '@apple'
    pair.1; // 13

    // With explicit type annotation
    let pair: (String, i32) = ("@pple", 13);

    // Destructure during assignment
    let (word, number) = ("glaive", 99);
    assert!(word, "glaive");
    assert!(number, 17);
}

// Declare a block to create a scope
// Blocks are expressions, meaning they evaluate to a value
fn pupil() {
    // This:
    let x = 42;
    // is equivalent to this:
    let y = {42};

    // A block is made up of a number of statements
    // The final expression in a block is the tail:
    x + y
    // The tail determines what the black will evaluate to
}
// These functions are equivalent:
fn get_number() -> u8 {
    return 55;
}
fn same_thing() -> u8 {
    55
}

// Double colons point to namespaces:
{
    use std::cmp::min;
    let least = std::cmp::min(3,8);
            // crate::source_file::function
    
    // Rust has `strict` scoping rules!
    // If you don't see it in your source code, it's not available.
}

// Types are namespaces, too,
//  and methods can be called as regular functions:
{
    let x = "brazen".len(); // 6
    let y = str::len("brazen"); // also 6
}

// STRUCTS
//   are declared with the struct keyword:
{
    struct Number {
        odd: bool,
        prime: bool,
        value: i32
    }
    // They can be initialized using literals:
    let x = Number { odd: false, prime: false, value: 18};
    let y = Number { odd: true, prime: true, value: 7};
}

// match arms can be used like a switch:
    // Can also match deeply nested data and de-structure it!

fn print_number(n: Number) {
    match n.value {
        1 => println!("One!"),
        2 => println!("Two!"),
        _ => println!("{}", n.value)
    }
    // The underscore "_" is/can be used as a 'catch-all' pattern
}

// implementations allow us to add methods to our custom types:
{
    struct Number {
        odd: bool,
        prime: bool,
        value: i32
    }
    impl Number {
        fn is_positive(&self) -> bool {
            self.value > 0
        }
    }
}

// Functions can be generic:
{
    // They can have multiple typed parameters,
    //   which can then be used in the function's declaration & body instead
    //   of concrete types!
    fn foo<T>(arg: T) {
        arg
    }
    fn bar<L, R>(left: L, right: R) {
        (left, right)
    }
}

// Vec is a heap-allocated array:
{
    let mut v1 = Vec::new();
    v1.push(2);
    let mut v2 = Vec::new();
    v2.push(false);
}

// The `vec!` macro essentially gives us "vec literals":
{
    fn my_vec_literals(){
        let v1: Vec<i16> = vec![1,2,3,4,5];
        let v2: Vec<bool> = vec![false, true, false, false, true];
    }
}

// A 'macro' essentially "expands" into regular code:
//    They can be recognized by the bang `!`
{
    // println! macro:
    println!{"{}", "Hello, world!"}

    // println! as "unfolded" code w/ the same effect:
    use std::io::{self, Write};
    io::stdout().lock().write_all(b"Hello, world!\n").unwrap();
}
// The panic! macro "violently" stops execution:
{
    panic!("Execution stopped at this line");
    // error: thread '{thread}' panicked at {panic_message}, {file}:{line_numbers}
}
// Some methods also panic:
{
    // For example, the Option type has variants `Some` or `None`,
    //   and if unwrap() is called on the None variant,
    //   the unwrap method panics:
    let option1: Option<i32> = None;
    option1.unwrap() // panics at the None value
}
// Option and Result are not structs, but enums:
{
    enum Option<T> {
        None,
        Some<T>
    }
    enum Result<T, E> {
        Ok(T),
        Err(E)
    }
}

// Rust uses a pattern of ERRORS as VALUES,
//   so we sometimes want to deliberately panic execution
// One way is to call unwrap() on the Error value,
// Another way is to call expect(error_message)
//      It is called `expect` because it telegraphs what
//      was to be expected when the result was unwrapped

// the Result enum can be `match`ed to handle the error:
{
    let melon_utf = &[240, 159, 141, 137];
    match str::from_utf8(melon_utf) {
        Ok(s) => println!("{}", s),
        Err(e) => panic!(e)
    }
    // or use `if let` to safely destructure the value,
    //   if it is `Ok`:
    if let Ok(s) = str::from_utf8(melon_utf) {
        println!("{}", s);
    }

    // OR INSTEAD, use the ? operator to do the exact same
    //   as the previous match statement:
    {
        let s = str::from_utf8(meon_utf)?;
        println!("{}", s);
        Ok(())
    }
}

// ITERATORS

// This one represents all natural numbers from 1 to Infinity:
{
    let natural_numbers = 1..;
}
// The most basic iterators are ranges like these:
{
    // Range of 0 or greater:
    (0..).contains(&100); // true

    // 20 or less than 20:
    (..=20).contains(&20); // true

    // only 3, 4, or 5:
    (3..6).contains(&4); //true
}

// ANYTHING iterable can be used in a `for` loop:
{
    // with a vec:
    for i in vec![52, 49, 21, 42, 55] {
        // do loopy stuff
    }
    // with a slice:
    for i in &[52, 49, 21] {
        // do loopy stuff
    }
    // with methods that are iterators:
    for c in "rust rules!".chars() {
        println!("Give me a {}", c);
    }
}

// Use an iterator in a `for` loop, even if the iterator items
//    are filtered, mapped, flattened, etc. :
{
    // this is the Fluent Interface pattern found everywhere in Rust (according to this lesson),
    //     not classes and mutations of shared data
    // Modeling program state as structs, and then writing functions to move between these valid states
    //   makes invalid states unrepresentable
    for c in "SuRPRISE INbOUND".chars().filter(|c| c.is_lowercase()).flat_map(|c| c.to_uppercase()) {
        print!("{}", c);
    }
    // output: UB

    
}
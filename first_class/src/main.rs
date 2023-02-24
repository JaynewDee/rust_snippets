//////////////////////////////
/// Functions as arguments
//////////////////////////////

fn main() {
    // Two functions passed through as first-class citizens,
    //    modifying the third argument (x) with consumption order
    //    defined in 'apply_functions' function
    let result = apply(add_one, double, 2);
    println!("Results of apply ::: {}", result);

    //
    // Pass as vector
    //

    let functions = Vec::from([add_one, double]);

    let results = apply_all(functions, 2);

    println!("Results of apply_all ::: {:?}", results);
}

fn reverse_sequence() {
    todo!()
}

//

fn apply_all(functions: Vec<fn(i32) -> i32>, x: i32) -> Vec<i32> {
    let mut results = Vec::new();

    for f in functions {
        results.push(f(x));
    }

    results
}

fn apply(f: fn(i32) -> i32, g: fn(i32) -> i32, x: i32) -> i32 {
    f(g(x))
}

//

fn add_one(x: i32) -> i32 {
    x + 1
}

//

fn double(x: i32) -> i32 {
    x * 2
}

//

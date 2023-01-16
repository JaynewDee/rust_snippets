use std::collections::HashMap;

/**
Rust version of most-frequent-word algorithm problem
*** */

fn main() {
    let vector = HashMap::from([
        (1, "joy"),
        (2, "awe"),
        (3, "grief"),
        (4, "inspiration"),
        (5, "despair"),
    ]);

    for (key, val) in &vector {
        println!("key: {key} val: {val} len: {}", &val.len())
    }
}

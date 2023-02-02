fn main() {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let mut names = vec!["Bob", "Frank", "Ferris"];

    // iter - This borrows each element of the collection through each iteration,
    //     thus leaving the collection untouched and available for reuse after the loop.
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    // into_iter - This consumes the collection so that on each iteration the exact data is provided.
    //     Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", names);

    //   iter_mut - This mutably borrows each element of the collection,
    //       allowing for the collection to be modified in place.
}

// If x : u8 = 0, what will happen when computing x - 1?
    // - It depends on the compiler mode.

    // Context: This expression will panic in debug mode and return 255 in release mode.


    use std::io;

    fn main() {
        let a = [1, 2, 3, 4, 5];
    
        println!("Please enter an array index.");
    
        let mut index = String::new();
    
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
    
        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");
    
        let element = a[index];
    
        println!("The value of the element at index {index} is: {element}");

        
        let t = ([1; 2], [3; 4]);
        let (a, _) = t;
        println!("{}", a[0] + t.1[0]); 
    }
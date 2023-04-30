// Implementing the Drop trait
//
// Specify the code to run when a value goes out of scope

struct MockSmartPointer {
    data: String,
}

impl Drop for MockSmartPointer {
    fn drop(&mut self) {
        println!("Drop fired for pointer exiting scope: {}", self.data);
    }
}

fn main() {
    let _first = MockSmartPointer {
        data: String::from("Some smart data."),
    };

    

    let _second = MockSmartPointer {
        data: String::from("Some more smart data!"),
    };

    println!("Pointers created and bout to Drop.");
}

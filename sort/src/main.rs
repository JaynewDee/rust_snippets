use PartialOrd;

fn main() {
    let mut vec_of_ints: Vec<i32> = vec![1, 5, 10, 2, 15];
    let mut vec_of_floats: Vec<f32> = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    // Sort ints with vec::sort or vec::sort_unstable
    vec_of_ints.sort();

    assert_eq!(vec_of_ints, vec![1, 2, 5, 10, 15]);

    // Sort f32 or f64 with vec::sort_by and PartialOrd::partial_cmp
    vec_of_floats.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec_of_floats, vec![1.1, 1.123, 1.15, 2.0, 5.5]);

    sort_vec_of_structs();
}

fn sort_vec_of_structs() {
    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        pub fn new(name: String, age: u32) -> Self {
            Person { name, age }
        }
    }

    let mut people = vec![
        Person::new("Trayvon".to_string(), 25),
        Person::new("Alyssa".to_string(), 1),
        Person::new("Hector".to_string(), 60),
    ];

    //Sort structs by derived natural order
    people.sort();
    assert_eq!(
        people,
        vec![
            Person::new("Alyssa".to_string(), 1),
            Person::new("Hector".to_string(), 60),
            Person::new("Trayvon".to_string(), 25),
        ]
    );
    // Sort people by age
    people.sort_by(|a, b| a.age.cmp(&b.age));

    assert_eq!(
        people,
        vec![
            Person::new("Alyssa".to_string(), 1),
            Person::new("Trayvon".to_string(), 25),
            Person::new("Hector".to_string(), 60),
        ]
    )
}

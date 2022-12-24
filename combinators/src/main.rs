#[derive(Debug)]

struct Student {
    name: String,
    age: i8
}

fn main() {
    let students: Vec<&str> = vec![
        "Arwen 17",
        "Jackson 24",
        "Lyla 28",
        "Everett 36",
        "Josephine 19"
    ];
    let students_vector: Vec<Student> = students.iter().flat_map(|student| split_pair(student)).collect();
    for student in students_vector.iter() {
        println!("Name: {:?}", student.name);
        println!("Age: {:?}", student.age)
    }
}

fn split_pair(student_str: &str) -> Option<Student> {
    let mut s = student_str.split(" ");
    let name = s.next()?.to_owned();
    let age = s.next()?.parse::<i8>().ok()?;
    Some(Student {name, age})
}

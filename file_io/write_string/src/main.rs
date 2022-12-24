use std::fs;

fn main() {
    let file_path: String = "my_file.txt".to_string();
    let contents: String = "We did it!".to_string();

    write_string_to_file(file_path, contents);
}

fn write_string_to_file(path: String, to_write: String) -> std::io::Result<()> {
    fs::write(path, to_write).unwrap();
    Ok(())
}

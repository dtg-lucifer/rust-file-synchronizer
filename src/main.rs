use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or("Cargo.toml".to_string());
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the contents of the file");

    println!("{contents}")
}

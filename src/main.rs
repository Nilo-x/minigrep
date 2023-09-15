use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let search_query = &args[2];

    println!("arg 1: {}", file_path);
    println!("arg 2: {}", search_query);

    let file_content = fs::read_to_string(file_path).expect("couldn't read file");

    let result = file_content.contains(search_query);

    if result {
        println!("file contains the word: \"{}\"", search_query);
    }
}

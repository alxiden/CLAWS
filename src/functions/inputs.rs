use std::io;

pub fn get_user_input(prompt: &str) -> String {

    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string();

    return input;
}
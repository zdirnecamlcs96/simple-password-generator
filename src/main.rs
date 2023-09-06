use std::fs::File;
use std::io::{self, BufRead};
use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    let password = generate_password();

    println!("\nNew Password: {}", password);
}

fn generate_words() -> Vec<String> {
    // Open the file for reading
    let file1 = File::open("./data/adjectives.txt").unwrap();
    let reader1 = io::BufReader::new(file1);

    let file2 = File::open("./data/nouns.txt").unwrap();
    let reader2 = io::BufReader::new(file2);

    // Init a empty list of string
    let mut lines: Vec<String> = Vec::new();

    // Add data from file to list
    for line in reader1.lines() {
        lines.push(line.unwrap());
    }

    for line in reader2.lines() {
        lines.push(line.unwrap());
    }

    let mut rng = rand::thread_rng();

    lines.shuffle(&mut rng);

    let words: Vec<String> = lines.into_iter().take(4).collect();

    words


}

fn shuffle_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    let mut rng = rand::thread_rng();
    chars.shuffle(&mut rng);
    chars.into_iter().collect()
}

fn generate_password() -> String{
    let words = generate_words();

    let mut password = String::new();

    print!("Secret Words: ");

    for (i, word) in words.iter().enumerate() {
        print!("{}", &word);

        password.push_str(shuffle_string(word.as_str()).as_str());

        // add dash if the word is not the first word or the last word
        if i != words.len()- 1{
            print!("|");
            password.push('-');
        }
    }

    let mut rng = rand::thread_rng();

    // List of special characters
    let special_chars = vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')'];

    // Decide how many special characters to add
    let num_special_chars = rng.gen_range(1..=3);

    // Add random special characters to the password
    for _ in 0..num_special_chars {
        if let Some(&special_char) = special_chars.choose(&mut rng) {
            password.push(special_char);
        }
    }

    // Generate a random number between 0 and 100
    let random_number = rng.gen_range(11..9999);

    // Add the random number to the password
    password.push_str(random_number.to_string().as_str());

    password

}

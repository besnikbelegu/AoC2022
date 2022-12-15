use std::env;
use std::fs::File;
use std::io::{self, Read};
fn read_file_locally(file_name: String) -> String {
    let working_dir: String = get_current_working_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    return working_dir + &"/".to_string() + &file_name;
}

fn get_current_working_dir() -> std::io::Result<std::path::PathBuf> {
    return env::current_dir();
}

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    // println!("Hello, world!");
    // println!(
    //     "Reading the file: {}",
    //     read_file_locally("data.txt".to_string())
    // );
    //read file
    let whole_file = filename_to_string(&read_file_locally("data.txt".to_string())).unwrap();
    // println!("Entire file content:{:?}", whole_file);

    //split by line
    let groups: Vec<&str> = whole_file.split("\n\n").collect();
    // let split_words: Vec<&str> = whole_file.split("\n").collect(); //words_by_line(&whole_file);
    // println!("All the split words: {:?}", groups);

    // sum elements inside the groups
    let mut sumed_elements: Vec<i32> = Vec::new();
    for e in groups {
        // println!("working with {:?} ", e);
        let e_vec: Vec<_> = e
            .split("\n")
            .map(|s| s.parse::<i32>().expect("parse error"))
            .collect();
        let sum: i32 = e_vec.iter().sum();
        // println!("values: {:?} sum: {:?}", e_vec, sum);
        sumed_elements.push(sum);
    }
    // println!("Summed values: {:?}", sumed_elements);
    sumed_elements.sort();
    //first part of the puzzle
    println!("And the winner is: {:?}", sumed_elements.iter().max());
    //second part of the puzzle
    sumed_elements.reverse();
    let sum_of_three: i32 = sumed_elements[0..3].iter().sum();
    println!(
        "And the top three winners are {:?} and total of all three is {:?}",
        &sumed_elements[0..3],
        sum_of_three
    );
}

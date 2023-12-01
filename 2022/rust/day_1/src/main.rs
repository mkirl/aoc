use std::fs;
use std::str::Split;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have read file");

    let mapped_contents = contents
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    println!("{}", mapped_contents.into_iter().rev().take(1).sum()::<u32>());
}

// fn main() {
//     let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
//     let split_contents: Split<&str> = contents.split("\n\n");
//
//     let mut mapped_contents = split_contents
//         .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum())
//         .collect::<Vec<u32>>();
//
//     mapped_contents.sort_unstable();
//     println!("{}", mapped_contents.into_iter().rev().take(1).sum::<u32>());
// }

use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("should have read file");

    let split_lines: Vec<String> = contents
        .split("\n")
        .into_iter()
        .map(|l| l.to_string())
        .collect();
    let mut total: i32 = 0;
    // let new_split = split_lines.map(|e| e.parse::<String>());

    for l in split_lines {
        let chars: Vec<_> = l.split_whitespace().collect();
        if chars.len() != 2 {
            continue; // skip invalid lines
        }

        let first_val = match chars[0].chars().next().unwrap_or_default() {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => -99999999, // skip invalid characters
        };

        let mut result = match chars[1].chars().next().unwrap_or_default() {
            'X' => loss(first_val),
            'Y' => first_val + 3,
            'Z' => win(first_val),
            _ => continue, // skip invalid characters
        };
        println!("{}", result);

        total += result;
    }

    println!("{}", total); // This should print 12
}

fn loss(num: i32) -> i32 {
    let mut value = (num + 2) % 3;
    return if value == 0 { 3 } else { value };
}

fn win(num: i32) -> i32 {
    let mut value = (num + 1) % 3;
    return if value == 0 { 9 } else { value + 6 };
}

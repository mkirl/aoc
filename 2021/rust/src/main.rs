fn main() {
    part1();
}

fn part1() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let lines: Vec<_> = include_str!("../inputs/01.in")
        .split("\n")
        .map(|s| s.parse::<i32>())
        .collect::<Result<_, _>>()
        .unwrap_or_else(|_| Vec::new());
    let mut count = 0;
    for (current, next) in lines.iter().zip(lines.clone().iter().skip(1)) {
        // println!("first: {} second {}", current, next);
        if current < next {
            count += 1;
        }
        println!("{}", count);
    }
    Ok(())
}

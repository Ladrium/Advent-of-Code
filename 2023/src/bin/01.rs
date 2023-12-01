fn main() {
    let input = aoc::get_lines("01.txt");         
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

fn part1(input: Vec<String>) -> u32 {
    println!("{:?}", input);
    let mut all: Vec<String> = vec![];

    for line in input {
        let sorted: Vec<char> = line.chars().filter(|el| el.is_numeric()).collect();
        let first = sorted.first().unwrap();
        let last = sorted.last().unwrap();
        all.push(format!("{first}{last}"));
    }
    all.iter().filter_map(|s| s.parse::<u32>().ok()).sum()
}

fn part2(input: Vec<String>) -> u32 {}

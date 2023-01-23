use rust_advent_of_code::AdventOfCode;

fn main() {
    let (maximum, top3) = AdventOfCode::day1_elf_calories();
    println!("day1_elf_calories: {maximum}, {top3}");
    let (day2_part1, day2_part2) = AdventOfCode::day2_rock_paper_scissors();
    println!("day2_rock_paper_scissors: {}, {}", day2_part1, day2_part2);
}

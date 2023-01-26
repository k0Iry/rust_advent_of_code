use rust_advent_of_code::AdventOfCode;

fn main() {
    let (maximum, top3) = AdventOfCode::day1_elf_calories();
    println!("day1_elf_calories: {maximum}, {top3}");

    let (day2_part1, day2_part2) = AdventOfCode::day2_rock_paper_scissors();
    println!("day2_rock_paper_scissors: {day2_part1}, {day2_part2}");

    let (day3_priority1, day3_priority2) = AdventOfCode::day3_sum_priorities();
    println!("day3_sum_priorities: {day3_priority1}, {day3_priority2}");

    let (day4_inclusive, day4_overlapped) = AdventOfCode::day4_fully_contained();
    println!("day4_fully_contained: {day4_inclusive}, day4_overlapped_pairs: {day4_overlapped}");
}

use rust_advent_of_code::{AdventOfCode, MarkerType};

fn main() {
    let (maximum, top3) = AdventOfCode::day1_elf_calories();
    println!("day1_elf_calories: {maximum}, {top3}");

    let (day2_part1, day2_part2) = AdventOfCode::day2_rock_paper_scissors();
    println!("day2_rock_paper_scissors: {day2_part1}, {day2_part2}");

    let (day3_priority1, day3_priority2) = AdventOfCode::day3_sum_priorities();
    println!("day3_sum_priorities: {day3_priority1}, {day3_priority2}");

    let (day4_inclusive, day4_overlapped) = AdventOfCode::day4_fully_contained();
    println!("day4_fully_contained: {day4_inclusive}, day4_overlapped_pairs: {day4_overlapped}");

    let (crate_stacks, crate_stacks2) = AdventOfCode::day5_rearrange_crates();
    println!("day5_rearrange_crates: {crate_stacks}, {crate_stacks2}");

    let position_of_packet_marker = AdventOfCode::day6_detect_packet_marker(MarkerType::Packet);
    let position_of_message_marker = AdventOfCode::day6_detect_packet_marker(MarkerType::Message);
    println!(
        "day6_detect_packet_marker: {position_of_packet_marker}, {position_of_message_marker}"
    );
    let (sum_of_size, sum_of_delete) = AdventOfCode::day7_sum_of_folders_size();
    println!("day 7: {sum_of_size}, {sum_of_delete}");

    let (day8_part1, day8_part2) = AdventOfCode::day8_visible_trees();
    println!("day 8: {day8_part1}, {day8_part2}");

    let day9_visited_pos_part1 = AdventOfCode::day9_rope_tail_visits(2);
    let day9_visited_pos_part2 = AdventOfCode::day9_rope_tail_visits(10);
    println!("day 9: {day9_visited_pos_part1}, {day9_visited_pos_part2}");

    let (day10_sig_strength, crt_screen) = AdventOfCode::day10_signal_strength_crt_screen();
    println!("day 10, signal strength: {day10_sig_strength}, and CRT screen:\n{crt_screen}");

    let day11_part1 = AdventOfCode::day11_monkey_business(20, false);
    let day11_part2 = AdventOfCode::day11_monkey_business(10000, true);
    println!("Day 11 monkey_business: part1: {day11_part1}, part2: {day11_part2}");

    let (day12_part1, day12_part2) = AdventOfCode::day12_best_signal();
    println!("Day 12 steps to get best signal: {day12_part1}, {day12_part2}");
}

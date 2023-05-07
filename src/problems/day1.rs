pub mod day1 {
    use std::{
        collections::BinaryHeap,
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    impl crate::AdventOfCode {
        pub fn day1_elf_calories() -> (i32, i32) {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day1-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());
            let mut min_heap = BinaryHeap::new();
            let mut calories = 0;
            file.lines().for_each(|line| {
                if let Ok(line) = line {
                    if let Ok(calorie) = line.parse::<i32>() {
                        calories += calorie
                    } else {
                        min_heap.push(-calories);
                        if min_heap.len() > 3 {
                            min_heap.pop();
                        }
                        calories = 0;
                    }
                }
            });
            let mut sum = 0;
            let mut max = 0;
            while let Some(top) = min_heap.pop() {
                sum += top;
                max = top;
            }
            (-max, -sum)
        }
    }
}

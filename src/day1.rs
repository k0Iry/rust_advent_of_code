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
            for line in file.lines() {
                let line = line.unwrap();
                if line.is_empty() {
                    min_heap.push(-calories);
                    if min_heap.len() > 3 {
                        min_heap.pop();
                    }
                    calories = 0;
                } else {
                    calories += line.parse::<i32>().unwrap()
                }
            }
            let mut sum = 0;
            let mut max = 0;
            while !min_heap.is_empty() {
                let top = min_heap.pop().unwrap();
                sum += top;
                max = top;
            }
            (-max, -sum)
        }
    }
}

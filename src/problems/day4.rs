pub mod day4 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    struct SectionRange {
        start: u32,
        end: u32,
    }

    impl SectionRange {
        fn new(start: u32, end: u32) -> Self {
            Self { start, end }
        }

        fn is_inclusive(&self, other: &Self) -> bool {
            if self.start > other.start {
                other.is_inclusive(self)
            } else {
                self.start == other.start || self.end >= other.end
            }
        }

        fn is_overlapped(&self, other: &Self) -> bool {
            !(self.end < other.start || other.end < self.start)
        }
    }
    impl crate::AdventOfCode {
        pub fn day4_fully_contained() -> (i32, i32) {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day4-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());

            let mut inclusive_pairs = 0;
            let mut overlapped_pairs = 0;
            for line in file.lines() {
                let line = line.unwrap();
                let pairs: Vec<&str> = line.split([',', '-']).collect();
                let pair1 = SectionRange::new(
                    pairs[0].parse::<u32>().unwrap(),
                    pairs[1].parse::<u32>().unwrap(),
                );
                let pair2 = SectionRange::new(
                    pairs[2].parse::<u32>().unwrap(),
                    pairs[3].parse::<u32>().unwrap(),
                );
                if pair1.is_overlapped(&pair2) {
                    if pair1.is_inclusive(&pair2) {
                        inclusive_pairs += 1;
                    }
                    overlapped_pairs += 1;
                }
            }

            (inclusive_pairs, overlapped_pairs)
        }
    }
}

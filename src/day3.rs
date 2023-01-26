pub mod day3 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    #[inline(always)]
    fn get_priority(item: char) -> i32 {
        match item {
            'a'..='z' => item as i32 - 'a' as i32 + 1,
            'A'..='Z' => 27 + item as i32 - 'A' as i32,
            _ => panic!("unsupported character: {item}"),
        }
    }

    impl crate::AdventOfCode {
        pub fn day3_sum_priorities() -> (i32, i32) {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day3-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());

            let mut priorities = 0;
            let mut priorities2 = 0;

            let mut count = 0;
            let mut alpha_groups = [0i8; 128];
            for line in file.lines() {
                count += 1;
                let line = line.unwrap();
                let mut alphas = [0i8; 128];
                for character in line.chars() {
                    if alphas[character as usize] == 0 {
                        alphas[character as usize] = 1;
                        if alpha_groups[character as usize] == count - 1 {
                            if count == 3 {
                                count = 0;
                                priorities2 += get_priority(character);
                                alpha_groups = [0i8; 128];
                                break;
                            } else {
                                alpha_groups[character as usize] = count
                            }
                        }
                    }
                }

                if line.len() % 2 == 1 {
                    panic!("A given rucksack always has the same number of items in each of its two compartments");
                }
                let (first, second) = line.split_at(line.len() / 2);
                alphas = [0i8; 128];
                for character in first.chars() {
                    alphas[character as usize] = 1;
                }
                for character in second.chars() {
                    if alphas[character as usize] == 1 {
                        priorities += get_priority(character);
                        break;
                    }
                }
            }
            (priorities, priorities2)
        }
    }
}

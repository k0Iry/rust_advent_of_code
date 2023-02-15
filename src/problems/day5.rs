pub mod day5 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    impl crate::AdventOfCode {
        pub fn day5_rearrange_crates() -> (String, String) {
            let mut stacks: Vec<String> = Vec::new();
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day5-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());

            let mut line_iter = file.lines();
            for line in line_iter.by_ref() {
                let line = line.unwrap();
                if line.is_empty() {
                    break;
                }
                for (index, c) in line.chars().enumerate() {
                    if (index + 3) % 4 == 0 {
                        let index = (index - 1) / 4;
                        if stacks.len() == index {
                            stacks.push(String::new());
                        }
                        if c.is_alphabetic() {
                            stacks[index].insert(0, c);
                        }
                    }
                }
            }

            let mut stacks2 = stacks.clone();
            for line in line_iter {
                let line = line.unwrap();
                let v = line.split(' ').collect::<Vec<&str>>();
                let mut number = v[1].parse::<u8>().unwrap();
                let mut number2 = number;
                let src = v[3].parse::<usize>().unwrap() - 1;
                let dst = v[5].parse::<usize>().unwrap() - 1;
                // part 1
                while number > 0 && !stacks[src].is_empty() {
                    if let Some(c) = stacks[src].pop() {
                        stacks[dst].push(c);
                        number -= 1;
                    }
                }

                // part 2
                let mut str = String::with_capacity(number2 as usize);
                while number2 > 0 && !stacks2[src].is_empty() {
                    if let Some(c) = stacks2[src].pop() {
                        str.push(c);
                        number2 -= 1;
                    }
                }
                while let Some(c) = str.pop() {
                    stacks2[dst].push(c);
                }
            }

            let mut result = String::with_capacity(stacks.len());
            for mut stack in stacks {
                result.push(stack.pop().unwrap_or('\0'))
            }
            let split_index = result.len();
            for mut stack in stacks2 {
                result.push(stack.pop().unwrap_or('\0'))
            }

            let (part1, part2) = result.split_at(split_index);
            (part1.to_owned(), part2.to_owned())
        }
    }
}

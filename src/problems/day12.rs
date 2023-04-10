pub mod day12 {
    use std::{
        collections::{HashSet, VecDeque},
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    const DIRECTIONS: [[i32; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

    impl crate::AdventOfCode {
        fn is_valid_next_step(cur: char, next: char) -> bool {
            match cur {
                'S' => next == 'a' || next == 'b',
                c => match next {
                    'E' => c == 'z' || c == 'y',
                    n => n == char::from_u32(c as u32 + 1).unwrap() || c >= n,
                },
            }
        }

        pub fn day12_best_signal() -> i32 {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day12-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());

            let mut height_map: Vec<Vec<char>> = vec![];
            let mut queue: VecDeque<i32> = VecDeque::default();

            for (i, line) in file.lines().enumerate() {
                height_map.push(vec![]);
                let last_map = height_map.last_mut().unwrap();
                let line = line.unwrap();
                for (j, ch) in line.char_indices() {
                    last_map.push(ch);
                    if ch == 'S' {
                        queue.push_back((i * line.len() + j) as i32);
                    }
                }
            }
            assert!(queue.len() == 1);
            let mut steps = 0;
            let row_len = height_map.len() as i32;
            let column_len = height_map[0].len() as i32;

            let mut visited: HashSet<i32> = HashSet::default();

            while !queue.is_empty() {
                let len = queue.len();
                for _ in 0..len {
                    let cur = queue.pop_front().unwrap();
                    let x = cur / column_len;
                    let y = cur % column_len;
                    let cur_ch = height_map[x as usize][y as usize];

                    if cur_ch == 'E' {
                        return steps;
                    }
                    if visited.contains(&cur) {
                        continue;
                    }
                    visited.insert(cur);

                    for dir in DIRECTIONS {
                        let new_x = x + dir[0];
                        let new_y = y + dir[1];
                        if new_x < 0
                            || new_y < 0
                            || new_x >= row_len
                            || new_y >= column_len
                            || visited.contains(&(new_x * column_len + new_y))
                            || !Self::is_valid_next_step(
                                cur_ch,
                                height_map[new_x as usize][new_y as usize],
                            )
                        {
                            continue;
                        }
                        queue.push_back(new_x * column_len + new_y);
                    }
                }
                steps += 1;
            }

            -1
        }
    }
}

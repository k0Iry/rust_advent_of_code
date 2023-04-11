pub mod day12 {
    use std::{
        collections::VecDeque,
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

        fn bfs(height_map: &Vec<Vec<char>>, mut queue: VecDeque<i32>) -> Option<i32> {
            let mut steps = 0;
            let row_len = height_map.len() as i32;
            let column_len = height_map[0].len() as i32;
            let mut visited = vec![0; (row_len * column_len) as usize];
            while !queue.is_empty() {
                let len = queue.len();
                for _ in 0..len {
                    let cur = queue.pop_front().unwrap();
                    let x = cur / column_len;
                    let y = cur % column_len;
                    let cur_ch = height_map[x as usize][y as usize];

                    if cur_ch == 'E' {
                        return Some(steps);
                    }
                    if *visited.get(cur as usize).unwrap() == 1 {
                        continue;
                    }
                    *visited.get_mut(cur as usize).unwrap() = 1;

                    for dir in DIRECTIONS {
                        let new_x = x + dir[0];
                        let new_y = y + dir[1];
                        if new_x < 0
                            || new_y < 0
                            || new_x >= row_len
                            || new_y >= column_len
                            || *visited.get((new_x * column_len + new_y) as usize).unwrap() == 1
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
            None
        }

        pub fn day12_best_signal() -> (i32, i32) {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day12-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());

            let mut height_map: Vec<Vec<char>> = vec![];
            let mut part1_start = -1;
            let mut part2_starts = vec![];

            for (i, line) in file.lines().enumerate() {
                height_map.push(vec![]);
                let last_map = height_map.last_mut().unwrap();
                let line = line.unwrap();
                for (j, ch) in line.char_indices() {
                    last_map.push(ch);
                    if ch == 'S' {
                        part1_start = (i * line.len() + j) as i32;
                    } else if ch == 'a' {
                        part2_starts.push((i * line.len() + j) as i32);
                    }
                }
            }
            assert!(part1_start != -1);
            assert!(!part2_starts.is_empty());
            let part1 = Self::bfs(&height_map, [part1_start].into_iter().collect()).unwrap();
            let mut part2 = i32::MAX;
            for start in part2_starts {
                if let Some(temp) = Self::bfs(&height_map, [start].into_iter().collect()) {
                    part2 = std::cmp::min(temp, part2);
                }
            }
            (part1, std::cmp::min(part1, part2))
        }
    }
}

pub mod day8 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    const SIDE_LENGTH: usize = 99;

    #[derive(Clone, Copy)]
    struct MaxValues {
        left_max: u8,
        right_max: u8,
        top_max: u8,
        bottom_max: u8,
    }

    impl MaxValues {
        fn is_not_visible(&self, height: u8) -> bool {
            self.left_max >= height
                && self.right_max >= height
                && self.top_max >= height
                && self.bottom_max >= height
        }
    }

    #[derive(Clone, Copy)]
    struct VisibleTrees {
        left_visible: u8,
        right_visible: u8,
        top_visible: u8,
        bottom_visible: u8,
    }

    impl VisibleTrees {
        fn scenic_score(&self) -> u32 {
            self.left_visible as u32
                * self.right_visible as u32
                * self.top_visible as u32
                * self.bottom_visible as u32
        }
    }

    impl crate::AdventOfCode {
        fn part1_visible_trees(tree_heights: &[[u8; SIDE_LENGTH]; SIDE_LENGTH]) -> u32 {
            let mut value_maps = [[MaxValues {
                left_max: 0,
                right_max: 0,
                top_max: 0,
                bottom_max: 0,
            }; SIDE_LENGTH]; SIDE_LENGTH];

            for row in 1..SIDE_LENGTH - 1 {
                for col in 1..SIDE_LENGTH - 1 {
                    value_maps[row][col].left_max = std::cmp::max(
                        tree_heights[row][col - 1],
                        value_maps[row][col - 1].left_max,
                    );
                    value_maps[row][SIDE_LENGTH - 1 - col].right_max = std::cmp::max(
                        tree_heights[row][SIDE_LENGTH - col],
                        value_maps[row][SIDE_LENGTH - col].right_max,
                    );

                    // this half may not be cache friendly though...
                    value_maps[col][row].top_max =
                        std::cmp::max(tree_heights[col - 1][row], value_maps[col - 1][row].top_max);
                    value_maps[SIDE_LENGTH - 1 - col][row].bottom_max = std::cmp::max(
                        tree_heights[SIDE_LENGTH - col][row],
                        value_maps[SIDE_LENGTH - col][row].bottom_max,
                    );
                }
            }
            let mut result = SIDE_LENGTH * SIDE_LENGTH;
            for row in 1..SIDE_LENGTH - 1 {
                for col in 1..SIDE_LENGTH - 1 {
                    if value_maps[row][col].is_not_visible(tree_heights[row][col]) {
                        result -= 1;
                    }
                }
            }
            result as u32
        }

        fn part2_scenic_scores(tree_heights: &[[u8; SIDE_LENGTH]; SIDE_LENGTH]) -> u32 {
            let mut scenic_scores = [[VisibleTrees {
                left_visible: 0,
                right_visible: 0,
                top_visible: 0,
                bottom_visible: 0,
            }; SIDE_LENGTH]; SIDE_LENGTH];

            for row in 1..SIDE_LENGTH - 1 {
                let mut mono_stack_right: Vec<usize> = vec![];
                let mut mono_stack_left: Vec<usize> = vec![];
                let mut mono_stack_bottom: Vec<usize> = vec![];
                let mut mono_stack_top: Vec<usize> = vec![];
                for col in 1..SIDE_LENGTH - 1 {
                    while !mono_stack_right.is_empty()
                        && tree_heights[row][*mono_stack_right.last().unwrap()]
                            <= tree_heights[row][col]
                    {
                        let prev = mono_stack_right.pop().unwrap();
                        scenic_scores[row][prev].right_visible = (col - prev) as u8;
                    }
                    mono_stack_right.push(col);

                    while !mono_stack_bottom.is_empty()
                        && tree_heights[*mono_stack_bottom.last().unwrap()][row]
                            <= tree_heights[col][row]
                    {
                        let prev = mono_stack_bottom.pop().unwrap();
                        scenic_scores[prev][row].bottom_visible = (col - prev) as u8;
                    }
                    mono_stack_bottom.push(col);

                    let col = SIDE_LENGTH - 1 - col;
                    while !mono_stack_left.is_empty()
                        && tree_heights[row][*mono_stack_left.last().unwrap()]
                            <= tree_heights[row][col]
                    {
                        let next = mono_stack_left.pop().unwrap();
                        scenic_scores[row][next].left_visible = (next - col) as u8;
                    }
                    mono_stack_left.push(col);

                    while !mono_stack_top.is_empty()
                        && tree_heights[*mono_stack_top.last().unwrap()][row]
                            <= tree_heights[col][row]
                    {
                        let next = mono_stack_top.pop().unwrap();
                        scenic_scores[next][row].top_visible = (next - col) as u8;
                    }
                    mono_stack_top.push(col);
                }
                while !mono_stack_right.is_empty() {
                    let prev = mono_stack_right.pop().unwrap();
                    scenic_scores[row][prev].right_visible = (SIDE_LENGTH - 1 - prev) as u8;
                }
                while !mono_stack_left.is_empty() {
                    let next = mono_stack_left.pop().unwrap();
                    scenic_scores[row][next].left_visible = next as u8;
                }
                while !mono_stack_bottom.is_empty() {
                    let prev = mono_stack_bottom.pop().unwrap();
                    scenic_scores[prev][row].bottom_visible = (SIDE_LENGTH - 1 - prev) as u8;
                }
                while !mono_stack_top.is_empty() {
                    let next = mono_stack_top.pop().unwrap();
                    scenic_scores[next][row].top_visible = next as u8;
                }
            }
            let mut result = 0;
            for scores in scenic_scores.iter().take(SIDE_LENGTH - 1).skip(1) {
                for score in scores.iter().take(SIDE_LENGTH - 1).skip(1) {
                    result = std::cmp::max(result, score.scenic_score());
                }
            }
            result
        }

        pub fn day8_visible_trees() -> (u32, u32) {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day8-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());
            let mut tree_maps = [[0u8; SIDE_LENGTH]; SIDE_LENGTH];
            for (row, line) in file.lines().enumerate() {
                let line = line.unwrap();
                for (col, c) in line.chars().enumerate() {
                    tree_maps[row][col] = (c as i32 - 0x30) as u8
                }
            }
            (
                Self::part1_visible_trees(&tree_maps),
                Self::part2_scenic_scores(&tree_maps),
            )
        }
    }
}

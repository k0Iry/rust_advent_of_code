pub mod day10 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    const ROWS: i32 = 6;
    const COLUMNS: i32 = 40;
    const TOTAL_PIXELS: i32 = ROWS * COLUMNS;

    pub struct Screen {
        crt: [[char; COLUMNS as usize]; ROWS as usize],
        sprite: i32,
        current_pos: i32,
    }

    impl std::fmt::Display for Screen {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for line in self.crt {
                writeln!(f, "{}", line.into_iter().collect::<String>())?;
            }
            Ok(())
        }
    }

    impl Screen {
        fn new() -> Self {
            Self {
                crt: [['.'; COLUMNS as usize]; ROWS as usize],
                current_pos: 0,
                sprite: 1,
            }
        }
        fn draw(&mut self) {
            if self.current_pos < TOTAL_PIXELS {
                let row = self.current_pos / COLUMNS;
                let cursor = self.current_pos % COLUMNS;
                if cursor.abs_diff(self.sprite) <= 1 {
                    self.crt[row as usize][cursor as usize] = '#';
                }
                self.current_pos += 1;
            }
        }
    }

    impl crate::AdventOfCode {
        pub fn day10_signal_strength_crt_screen() -> (i32, Screen) {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day10-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());
            let mut reg_val = 1;
            let mut circles = 1;

            let mut screen = Screen::new();

            let mut strength = 0;
            for line in file.lines() {
                let line = line.unwrap();
                let splits: Vec<&str> = line.split(' ').collect();
                let (mut steps, adding) = if splits.len() == 1 {
                    (1, 0)
                } else {
                    (2, splits[1].parse::<i32>().unwrap())
                };
                while steps > 0 {
                    if (circles - 20) % 40 == 0 {
                        strength += circles * reg_val
                    }
                    circles += 1;
                    steps -= 1;
                    screen.draw();
                }
                reg_val += adding;
                screen.sprite += adding;
            }
            (strength, screen)
        }
    }
}

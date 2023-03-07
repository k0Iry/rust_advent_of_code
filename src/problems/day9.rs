pub mod day9 {
    use std::{
        collections::HashSet, fs::File, io::BufRead, io::BufReader, path::PathBuf, str::FromStr,
    };

    #[derive(PartialEq, Eq, Hash, Default, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[repr(i8)]
    #[derive(Clone, Copy)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        fn offset(self) -> i32 {
            match self {
                Self::Up | Self::Right => 1,
                _ => -1,
            }
        }
    }

    impl FromStr for Direction {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "U" => Ok(Direction::Up),
                "D" => Ok(Direction::Down),
                "L" => Ok(Direction::Left),
                "R" => Ok(Direction::Right),
                _ => Err(()),
            }
        }
    }

    #[derive(Default)]
    struct Rope {
        head: Point,
        tail: Point,
    }

    impl Rope {
        fn need_to_move(&self) -> bool {
            if self.head.x == self.tail.x {
                (self.head.y - self.tail.y).abs() > 1
            } else if self.head.y == self.tail.y {
                (self.head.x - self.tail.x).abs() > 1
            } else {
                (self.head.x - self.tail.x).abs() + (self.head.y - self.tail.y).abs() > 2
            }
        }
        fn move_forward(&mut self, dir: Direction) {
            match dir {
                Direction::Up | Direction::Down => {
                    self.head.y += dir.offset();
                    if self.need_to_move() {
                        self.tail.y = self.head.y - dir.offset();
                        self.tail.x = self.head.x;
                    }
                }
                _ => {
                    self.head.x += dir.offset();
                    if self.need_to_move() {
                        self.tail.x = self.head.x - dir.offset();
                        self.tail.y = self.head.y;
                    }
                }
            }
        }
    }

    impl crate::AdventOfCode {
        pub fn day9_rope_tail_visits() -> u32 {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day9-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());

            let mut rope = Rope::default();
            let mut visited: HashSet<Point> = HashSet::new();
            for line in file.lines() {
                let line = line.unwrap();
                let splits: Vec<&str> = line.split(' ').collect();

                let dir = Direction::from_str(splits.first().unwrap()).unwrap();
                let steps = splits.last().unwrap().parse::<u8>().unwrap();
                for _ in 0..steps {
                    rope.move_forward(dir);
                    visited.insert(rope.tail);
                }
            }

            visited.len() as u32
        }
    }
}

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

    struct Rope {
        knots: Vec<Point>,
    }

    impl Rope {
        fn new(num_knots: usize) -> Self {
            Self {
                knots: vec![Point::default(); num_knots],
            }
        }
        fn move_forward(&mut self, dir: Direction) {
            let mut knot_iter = self.knots.iter_mut();

            let mut cur = knot_iter.next().take().unwrap();
            match dir {
                Direction::Up | Direction::Down => cur.y += dir.offset(),
                _ => cur.x += dir.offset(),
            }

            let mut next = knot_iter.next();

            while next.is_some() {
                let nex = next.unwrap();
                let diff_x = (nex.x - cur.x).abs();
                let diff_y = (nex.y - cur.y).abs();
                if diff_x > 1 || diff_y > 1 {
                    let offset_x = std::cmp::min(1, diff_x);
                    let offset_y = std::cmp::min(1, diff_y);
                    if nex.x < cur.x {
                        nex.x += offset_x
                    } else {
                        nex.x -= offset_x
                    }
                    if nex.y < cur.y {
                        nex.y += offset_y
                    } else {
                        nex.y -= offset_y
                    }
                } else {
                    break;
                }
                cur = nex;
                next = knot_iter.next();
            }
        }
    }

    impl crate::AdventOfCode {
        pub fn day9_rope_tail_visits(num_knots: usize) -> u32 {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day9-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());

            let mut rope = Rope::new(num_knots);
            let mut visited: HashSet<Point> = HashSet::new();
            for line in file.lines() {
                let line = line.unwrap();
                let splits: Vec<&str> = line.split(' ').collect();

                let dir = Direction::from_str(splits.first().unwrap()).unwrap();
                let steps = splits.last().unwrap().parse::<u8>().unwrap();
                for _ in 0..steps {
                    rope.move_forward(dir);
                    visited.insert(*rope.knots.last().unwrap());
                }
            }

            visited.len() as u32
        }
    }
}

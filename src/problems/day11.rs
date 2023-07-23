pub mod day11 {
    use std::{
        cell::RefCell,
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
        str::FromStr,
    };

    #[derive(Clone, Copy /*, Debug */)]
    enum Operation {
        Plus(i32),
        Multiply(i32),
        Unknown,
    }

    impl FromStr for Operation {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut splits: Vec<&str> = s.split_whitespace().collect();
            let worry = splits.pop().unwrap();
            let operation = splits.pop().unwrap();
            match (operation, worry) {
                ("*", worry) => Ok(Self::Multiply(worry.parse::<i32>().unwrap_or(-1))),
                ("+", worry) => Ok(Self::Plus(worry.parse::<i32>().unwrap_or(-1))),
                _ => panic!("unsupported"),
            }
        }
    }

    struct Monkey {
        items: RefCell<Vec<i32>>,
        operation: Operation,
        divisor: i32,
        neighbors: [i32; 2],
        inspects: RefCell<i32>,
    }

    impl Operation {
        fn operate_on_item(self, item: i32, lcm: Option<i32>) -> i32 {
            match self {
                Operation::Plus(x) => {
                    if x < 0 {
                        if lcm.is_some() {
                            2 * item
                        } else {
                            ((2 * item) as f32 / 3.0).floor() as i32
                        }
                    } else if let Some(lcm) = lcm {
                        (item + x) % lcm
                    } else {
                        ((item + x) as f32 / 3.0).floor() as i32
                    }
                }
                Operation::Multiply(x) => {
                    if x < 0 {
                        if let Some(lcm) = lcm {
                            ((item as i64 * item as i64) % lcm as i64) as i32
                        } else {
                            ((item as f32 * item as f32) / 3.0).floor() as i32
                        }
                    } else if lcm.is_some() {
                        item * x
                    } else {
                        ((item * x) as f32 / 3.0).floor() as i32
                    }
                }
                _ => panic!("unsupported operation"),
            }
        }
    }

    impl Monkey {
        fn take_round(&self, lcm: Option<i32>) -> Vec<(i32, i32)> {
            let mut items_to_sent = vec![];
            *self.inspects.borrow_mut() += self.items.borrow().len() as i32;
            for item in &*self.items.borrow() {
                let worry_level = self.operation.operate_on_item(*item, lcm);
                if worry_level % self.divisor == 0 {
                    items_to_sent.push((worry_level, self.neighbors[0]))
                } else {
                    items_to_sent.push((worry_level, self.neighbors[1]))
                }
            }
            self.items.borrow_mut().clear();
            items_to_sent
        }
    }

    struct Monkeys(Vec<Monkey>);

    impl Monkeys {
        fn take_rounds(&self, num_rounds: i32, lcm: Option<i32>) {
            for _ in 0..num_rounds {
                for monkey in &self.0 {
                    let distribution_plans = monkey.take_round(lcm);
                    for (worry_level, neighbor) in distribution_plans {
                        self.0
                            .get(neighbor as usize)
                            .unwrap()
                            .items
                            .borrow_mut()
                            .push(worry_level);
                    }
                }
            }
        }
    }

    impl crate::AdventOfCode {
        pub fn day11_monkey_business(num_rounds: i32, no_relief: bool) -> i64 {
            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day11-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());

            let mut monkeys = vec![];
            let mut items = vec![];
            let mut operation = Operation::Unknown;
            let mut neighbors = [0; 2];
            let mut divisor = 0;
            let mut lcm = 1;
            for line in file.lines() {
                let line = line.unwrap();

                let splits: Vec<&str> = line.trim().split(':').collect();
                if splits[0].eq("Starting items") {
                    items = splits[1]
                        .split(',')
                        .map(|item| item.trim().parse::<i32>().unwrap())
                        .collect();
                } else if splits[0].eq("Operation") {
                    operation = Operation::from_str(splits[1]).unwrap();
                } else if splits[0].eq("Test") {
                    divisor = splits[1]
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();
                    lcm *= divisor;
                } else if splits[0].starts_with("If true") {
                    neighbors[0] = splits[1]
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();
                } else if splits[0].starts_with("If false") {
                    neighbors[1] = splits[1]
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();
                    monkeys.push(Monkey {
                        items: RefCell::new(items.clone()),
                        operation,
                        neighbors,
                        divisor,
                        inspects: RefCell::new(0),
                    })
                }
            }

            let monkeys = Monkeys(monkeys);
            monkeys.take_rounds(num_rounds, if no_relief { Some(lcm) } else { None });

            let mut max = 0;
            let mut second_max = 0;
            for monkey in monkeys.0 {
                let inspect = *monkey.inspects.borrow();
                if max <= inspect {
                    second_max = max;
                    max = inspect;
                } else if inspect > second_max && inspect != max {
                    second_max = inspect;
                }
            }
            (max as i64) * (second_max as i64)
        }
    }
}

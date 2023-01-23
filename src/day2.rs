pub mod day2 {
    use std::{path::PathBuf, io::{BufReader, BufRead}, fs::File};

    #[derive(PartialEq)]
    enum GameScores {
        Lose,
        Draw,
        Win,
    }
    impl GameScores {
        fn new(result: &str) -> Self {
            match result {
                "X" => GameScores::Lose,
                "Y" => GameScores::Draw,
                _ => GameScores::Win,
            }
        }

        fn value(&self) -> i32 {
            match self {
                GameScores::Lose => 0,
                GameScores::Draw => 3,
                _ => 6,
            }
        }
    }

    #[derive(PartialEq, Clone, Copy)]
    enum GameOptions {
        Rock,
        Paper,
        Scissors,
    }
    impl GameOptions {
        fn new(option: &str) -> Self {
            match option {
                "A" | "X" => GameOptions::Rock,
                "B" | "Y" => GameOptions::Paper,
                _ => GameOptions::Scissors,
            }
        }

        fn value(&self) -> i32 {
            match self {
                GameOptions::Rock => 1,
                GameOptions::Paper => 2,
                GameOptions::Scissors => 3,
            }
        }

        fn scores_with_opponent(self, opponent: &Self) -> i32 {
            let base = self.value();
            match (self, opponent) {
                (Self::Rock, Self::Paper)
                | (Self::Paper, Self::Scissors)
                | (Self::Scissors, Self::Rock) => base + GameScores::Lose.value(),
                (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper) => base + GameScores::Win.value(),
                _ => base + GameScores::Draw.value(),
            }
        }

        fn scores_with_result(self, result: GameScores) -> i32 {
            let score = result.value();
            match self {
                Self::Rock => match result {
                    GameScores::Lose => Self::Scissors.value() + score,
                    GameScores::Win => Self::Paper.value() + score,
                    GameScores::Draw => Self::Rock.value() + score,
                },
                Self::Paper => match result {
                    GameScores::Lose => Self::Rock.value() + score,
                    GameScores::Win => Self::Scissors.value() + score,
                    GameScores::Draw => Self::Paper.value() + score,
                },
                Self::Scissors => match result {
                    GameScores::Lose => Self::Paper.value() + score,
                    GameScores::Win => Self::Rock.value() + score,
                    GameScores::Draw => Self::Scissors.value() + score,
                },
            }
        }
    }

    impl crate::AdventOfCode {
        pub fn day2_rock_paper_scissors() -> (i32, i32) {
            let mut scores = 0;
            let mut scores2 = 0;

            let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            file_path.push("inputs/day2-input.txt");
            let file = BufReader::new(File::open(file_path).unwrap());

            for line in file.lines() {
                let line = line.unwrap();
                let v: Vec<&str> = line.split(' ').collect();
                let me = GameOptions::new(v[1]);
                let opponent = GameOptions::new(v[0]);
                scores += me.scores_with_opponent(&opponent);
                scores2 += opponent.scores_with_result(GameScores::new(v[1]));
            }

            (scores, scores2)
        }
    }
}

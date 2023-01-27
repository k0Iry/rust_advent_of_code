pub mod day2 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

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

    #[derive(PartialEq)]
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
            match (self, opponent) {
                (Self::Rock, Self::Paper)
                | (Self::Paper, Self::Scissors)
                | (Self::Scissors, Self::Rock) => GameScores::Lose.value(),
                (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper) => GameScores::Win.value(),
                _ => GameScores::Draw.value(),
            }
        }

        fn scores_with_result(self, result: GameScores) -> i32 {
            match (self, result) {
                (Self::Rock, GameScores::Draw)
                | (Self::Paper, GameScores::Lose)
                | (Self::Scissors, GameScores::Win) => Self::Rock.value(),
                (Self::Rock, GameScores::Win)
                | (Self::Paper, GameScores::Draw)
                | (Self::Scissors, GameScores::Lose) => Self::Paper.value(),
                _ => Self::Scissors.value(),
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
                scores += me.value() + me.scores_with_opponent(&opponent);
                let round_score = GameScores::new(v[1]);
                scores2 += round_score.value() + opponent.scores_with_result(round_score);
            }

            (scores, scores2)
        }
    }
}

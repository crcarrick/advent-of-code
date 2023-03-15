use core::str::FromStr;

#[derive(Copy, Clone)]
enum Action {
    Rock = 1,
    Paper,
    Scissor,
}

impl Action {
    fn from_turn(turn: &str) -> Action {
        match turn {
            "A" | "X" => Action::Rock,
            "B" | "Y" => Action::Paper,
            "C" | "Z" => Action::Scissor,
            _ => panic!("Unknown turn: {}", turn),
        }
    }
}

pub fn solve_part_a(input: &str) -> String {
    let score = input
        .lines()
        .map(|r| {
            let round = r
                .split_whitespace()
                .map(|turn| Action::from_turn(turn))
                .collect::<Vec<_>>();

            let outcome = match &round[..] {
                [Action::Rock, Action::Scissor]
                | [Action::Paper, Action::Rock]
                | [Action::Scissor, Action::Paper] => 0,
                [Action::Rock, Action::Paper]
                | [Action::Paper, Action::Scissor]
                | [Action::Scissor, Action::Rock] => 6,
                [Action::Rock, Action::Rock]
                | [Action::Paper, Action::Paper]
                | [Action::Scissor, Action::Scissor] => 3,
                _ => panic!("Expected 2 actions"),
            };

            return outcome + round[1] as u32;
        })
        .sum::<u32>();

    score.to_string()
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Action, Self::Err> {
        match s {
            "A" => Ok(Action::Rock),
            "B" => Ok(Action::Paper),
            "C" => Ok(Action::Scissor),
            _ => Err("Unexpected action".to_string()),
        }
    }
}

impl Action {
    fn to_losing_move(&self) -> Action {
        match self {
            Action::Rock => Action::Scissor,
            Action::Paper => Action::Rock,
            Action::Scissor => Action::Paper,
        }
    }

    fn to_winning_move(&self) -> Action {
        match self {
            Action::Rock => Action::Paper,
            Action::Paper => Action::Scissor,
            Action::Scissor => Action::Rock,
        }
    }
}

// notes
//
//

// ============================================================================================================================== //
// ============================================================================================================================== //
// ============================================================================================================================== //

pub fn solve_part_b(input: &str) -> String {
    let total = input
        .lines()
        .map(|round| {
            let turn = round.split_whitespace().collect::<Vec<_>>();
            let theirs = turn[0].parse::<Action>().unwrap();

            return match turn[1] {
                "X" => 0 + theirs.to_losing_move() as u32,
                "Y" => 3 + theirs as u32,
                "Z" => 6 + theirs.to_winning_move() as u32,
                _ => panic!("Unexpected desired outcome {}", turn[1]),
            };
        })
        .sum::<u32>();

    total.to_string()
}

// notes
//
// i could have implemented PartialOrd on Action

#[cfg(test)]
mod test {
    use super::{solve_part_a, solve_part_b};

    const INPUT: &str = "A Y
B X
C Z
";

    #[test]
    fn test_part_a() {
        let result = solve_part_a(INPUT);

        assert_eq!(result, "15");
    }

    #[test]
    fn test_part_b() {
        let result = solve_part_b(INPUT);

        assert_eq!(result, "12");
    }
}

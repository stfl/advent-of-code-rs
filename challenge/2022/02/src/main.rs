use std::str::FromStr;

use fetch::Input;

#[derive(PartialEq, Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Shape::*;
        let shape = match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => return Err(String::from("Invalid Shape")),
        };
        Ok(shape)
    }
}

impl Shape {
    fn fight(&self, other: &Shape) -> u32 {
        use Shape::*;
        match (self, other) {
            (a, b) if a == b => 3,
            (Rock, Paper) => 0,
            (Rock, Scissors) => 6,
            (Paper, Rock) => 6,
            (Paper, Scissors) => 0,
            (Scissors, Paper) => 6,
            (Scissors, Rock) => 0,
            _ => unreachable!(),
        }
    }

    fn self_score(&self) -> u32 {
        use Shape::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn score_round(&self, other: &Shape) -> u32 {
        self.self_score() + self.fight(other)
    }

    fn shape_by_outcome(&self, outcome: &Outcome) -> Shape {
        use Outcome::*;
        use Shape::*;

        match (self, outcome) {
            (&ref a, Draw) => (*a).clone(),
            (Rock, Win) => Paper,
            (Rock, Lose) => Scissors,
            (Paper, Win) => Scissors,
            (Paper, Lose) => Rock,
            (Scissors, Win) => Rock,
            (Scissors, Lose) => Paper,
        }
    }
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Outcome::*;
        let outcome = match s {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => return Err(String::from("Invalid Outcome")),
        };
        Ok(outcome)
    }
}

fn score_row2(row: Vec<&str>) -> u32 {
    assert_eq!(row.len(), 2, "Expecting 2 elements per row");

    let other = Shape::from_str(row[0]).unwrap();
    let outcome = Outcome::from_str(row[1]).unwrap();

    outcome.score() + other.shape_by_outcome(&outcome).self_score()
}

fn calc_score1(input: &str) -> u32 {
    input
        .split_terminator("\n")
        .map(|x| {
            x.split_whitespace()
                .map(|s| Shape::from_str(s).unwrap())
                .collect::<Vec<Shape>>()
            // .try_into::<[Shape; 2]>()?
        })
        .map(|x| x[1].score_round(&x[0]))
        .sum()
}

fn calc_score2(input: &str) -> u32 {
    input
        .split_terminator("\n")
        .map(|x| x.split_whitespace().collect())
        .map(score_row2)
        .sum()
}

fn main() {
    let input = Input::try_get(2022, 2).unwrap();
    let res = calc_score1(&input.text);
    let res2 = calc_score2(&input.text);
    println!("{res}\n{res2}");
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    static EXAMPLE_INPUT: &str = "A Y
B X
C Z";

    static EXAMPLE_RESULT: u32 = 15;
    static EXAMPLE_RESULT2: u32 = 12;

    #[test]
    fn test_example() {
        let res: u32 = calc_score1(EXAMPLE_INPUT);

        assert_eq!(res, EXAMPLE_RESULT);
    }

    #[test]
    fn test_example2() {
        let input = EXAMPLE_INPUT;

        let res: u32 = calc_score2(input);
        assert_eq!(res, EXAMPLE_RESULT2);
    }
}

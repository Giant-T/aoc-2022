#[derive(PartialEq, Clone, Copy, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug)]
struct Match {
    pub opponent_shape: Shape,
    pub player_shape: Shape,
}

impl Match {
    fn calculate_outcome(&self) -> Outcome {
        let difference: i32 = self.player_shape as i32 - self.opponent_shape as i32;

        if difference == 0 {
            return Outcome::Draw;
        } else if difference == -1 || difference == 2 {
            return Outcome::Lost;
        } else {
            return Outcome::Win;
        }
    }

    pub fn calculate_score(&self) -> u32 {
        self.calculate_outcome() as u32 + self.player_shape as u32
    }
}

fn calculate_move(opponent_move: &Shape, outcome: &Outcome) -> Shape {
    match outcome {
        Outcome::Lost => match opponent_move {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
        Outcome::Draw => opponent_move.clone(),
        Outcome::Win => match opponent_move {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
    }
}

fn main() {
    let mut score = 0;

    include_str!("../inputs.txt").lines().for_each(|line| {
        let moves: Vec<&str> = line.split(" ").collect();

        let opponent_move = match moves[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!(),
        };

        let outcome = match moves[1] {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!(),
        };

        let current_match = Match {
            opponent_shape: opponent_move,
            player_shape: calculate_move(&opponent_move, &outcome),
        };

        score += current_match.calculate_score();
    });

    println!("The score is {}.", score);
}

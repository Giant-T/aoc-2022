#[derive(PartialEq, Clone, Copy, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
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

fn main() {
    let mut score = 0;

    include_str!("../inputs.txt").lines().for_each(|line| {
        let moves: Vec<Shape> = line
            .split(" ")
            .map(|letter| match letter {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Scissors,
                _ => panic!(),
            })
            .collect();

        let current_match = Match {
            opponent_shape: moves[0],
            player_shape: moves[1],
        };

        score += current_match.calculate_score();
    });

    println!("The score is {}.", score);
}

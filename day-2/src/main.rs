use std::fs;

#[derive(Copy, Clone, Debug)]
enum Game {
    Rock,
    Paper,
    Scissors,
}

struct Input {
    choice: Game,
}

impl From<char> for Input {
    fn from(choice: char) -> Self {
        let choice: String = choice.into();

        if choice == "A" || choice == "X" {
            return Input { choice: Game::Rock };
        }

        if choice == "B" || choice == "Y" {
            return Input {
                choice: Game::Paper,
            };
        }

        if choice == "C" || choice == "Z" {
            return Input {
                choice: Game::Scissors,
            };
        }

        Input { choice: Game::Rock }
    }
}
#[derive(Debug)]
struct Match(Game, Game);

impl From<&str> for Match {
    fn from(input: &str) -> Self {
        let current_match: Vec<Game> = input
            .chars()
            .filter(|char| !char.is_whitespace())
            .map(|input| Input::from(input).choice)
            .collect();

        if current_match.len() == 2 {
            let current_match = &current_match[0..2];
            let oponent_game = current_match.get(0).unwrap();
            let mine_game = current_match.get(1).unwrap();
            let current_match = Match(*oponent_game, *mine_game);
            println!(
                "{:?}, Points: {} ",
                current_match,
                current_match.calculate()
            );
            return current_match;
        }

        Match(Game::Rock, Game::Rock)
    }
}

const DRAW: u32 = 3;
const WIN: u32 = 6;

impl Match {
    fn calculate(&self) -> u32 {
        match self.0 {
            Game::Rock => match self.1 {
                Game::Rock => 1 + DRAW,
                Game::Paper => 2 + WIN,
                Game::Scissors => 3,
            },
            Game::Paper => match self.1 {
                Game::Rock => 1,
                Game::Paper => 2 + DRAW,
                Game::Scissors => 3 + WIN,
            },
            Game::Scissors => match self.1 {
                Game::Rock => 1 + WIN,
                Game::Paper => 2,
                Game::Scissors => 3 + DRAW,
            },
        }
    }

    fn calculate_part2(&self) -> u32 {
        match self.0 {
            Game::Rock => match self.1 {
                Game::Rock => 3,
                Game::Paper => 1 + DRAW,
                Game::Scissors => 2 + WIN,
            },
            Game::Paper => match self.1 {
                Game::Rock => 1,
                Game::Paper => 2 + DRAW,
                Game::Scissors => 3 + WIN,
            },
            Game::Scissors => match self.1 {
                Game::Rock => 2,
                Game::Paper => 3 + DRAW,
                Game::Scissors => 1 + WIN,
            },
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let binding = &fs::read("./day-2/input")?;
    let content = String::from_utf8_lossy(binding);
    let content: Vec<&str> = content.split("\n").collect();
    let matches: Vec<Match> = content.iter().map(|item| Match::from(*item)).collect();
    let total = matches
        .iter()
        .map(|item| item.calculate_part2())
        .reduce(|acc, current| acc + current);
    println!("{:?}", total.unwrap());
    Ok(())
}

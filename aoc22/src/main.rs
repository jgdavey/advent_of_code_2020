use std::collections::VecDeque;
use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut game: Game = input.parse().unwrap();
    game.play();
    println!("Rounds: {}, Score: {}", game.round, game.score().unwrap());
}

#[derive(Debug)]
struct Game {
    round: usize,
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
}

impl FromStr for Game {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut players = s.split("\n\n").map(|player| {
            player
                .lines()
                .skip(1)
                .map(|l| l.parse().map_err(|_| "bad number"))
                .collect::<Result<VecDeque<_>,_>>()
        });
        let player1 = players.next().ok_or("missing player")??;
        let player2 = players.next().ok_or("missing player")??;
        Ok(Game {
            round: 0,
            player1,
            player2,
        })
    }
}

impl Game {
    fn next_round(&mut self) -> Result<usize, usize> {
        if self.player1.is_empty() || self.player2.is_empty() {
            return Err(self.round);
        }

        let a = self.player1.pop_front().unwrap();
        let b = self.player2.pop_front().unwrap();

        if a > b {
            self.player1.push_back(a);
            self.player1.push_back(b);
        } else {
            self.player2.push_back(b);
            self.player2.push_back(a);
        }

        self.round += 1;

        Ok(self.round)
    }

    fn score(&self) -> Option<usize> {
        let winner = if self.player1.is_empty() {
            &self.player2
        } else if self.player2.is_empty() {
            &self.player1
        } else {
            return None;
        };

        Some(winner.iter().rev().enumerate().map(|(i, val)| (i + 1) * val).sum())
    }

    fn play(&mut self) -> usize {
        while let Ok(_) = self.next_round() {}
        self.score().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\
                     \n\
                     Player 2:\n5\n8\n4\n7\n10";

        let game: Game = input.parse().unwrap();
        assert_eq!(game.player1, VecDeque::from(vec![9,2,6,3,1]));
        assert_eq!(game.player2, VecDeque::from(vec![5,8,4,7,10]));
    }

    #[test]
    fn test_play() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\
                     \n\
                     Player 2:\n5\n8\n4\n7\n10";

        let mut game: Game = input.parse().unwrap();
        assert_eq!(game.next_round(), Ok(1));
        assert_eq!(game.player1, VecDeque::from(vec![2,6,3,1,9,5]));
        assert_eq!(game.player2, VecDeque::from(vec![8,4,7,10]));

        game.play();
        assert_eq!(game.round, 29);
        assert_eq!(game.score(), Some(306))
    }
}

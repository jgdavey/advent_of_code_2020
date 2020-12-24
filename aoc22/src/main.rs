use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut game: Game = input.parse().unwrap();
    println!("{}", game.summary());
    let result = game.play();
    println!("{}", game.summary());
    println!(
        "Result: {:?}, Rounds: {}, Score: {}",
        result,
        game.round,
        game.score().unwrap()
    );
}

#[derive(Debug)]
struct Game {
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
    round: usize,
    seen: HashSet<(VecDeque<usize>, VecDeque<usize>)>,
}

impl FromStr for Game {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut players = s.split("\n\n").map(|player| {
            player
                .lines()
                .skip(1)
                .map(|l| l.parse().map_err(|_| "bad number"))
                .collect::<Result<VecDeque<_>, _>>()
        });
        let player1 = players.next().ok_or("missing player")??;
        let player2 = players.next().ok_or("missing player")??;
        Ok(Game::new(player1, player2))
    }
}

#[derive(Debug, PartialEq)]
enum Player {
    Player1,
    Player2,
}

impl Game {
    fn summary(&self) -> String {
        format!(
            "Game({}) after round {}:\nPlayer 1 {:?}\nPlayer 2 {:?}",
            self.player1.len() + self.player2.len(),
            self.round,
            self.player1,
            self.player2
        )
    }

    fn new(player1: VecDeque<usize>, player2: VecDeque<usize>) -> Self {
        //println!("p1: {}, p2: {}", player1.len(), player2.len());
        Game {
            player1,
            player2,
            round: 0,
            seen: HashSet::new(),
        }
    }

    fn game_over(&self) -> bool {
        self.player1.is_empty() || self.player2.is_empty()
    }

    fn next_round(&mut self) -> Result<Player, usize> {
        use Player::*;

        if self.game_over() {
            return Err(self.round);
        }
        self.round += 1;

        let state = (self.player1.clone(), self.player2.clone());

        if self.seen.contains(&state) || self.round > 1000 {
            self.player2.clear();
            return Ok(Player1);
        }

        self.seen.insert(state);

        let p1 = self.player1.pop_front().unwrap();
        let p2 = self.player2.pop_front().unwrap();

        let (player, cards) = if self.player1.len() >= p1 && self.player2.len() >= p2 {
            let mut subgame = Game::new(
                self.player1
                    .iter()
                    .take(p1)
                    .cloned()
                    .collect::<VecDeque<_>>(),
                self.player2
                    .iter()
                    .take(p2)
                    .cloned()
                    .collect::<VecDeque<_>>(),
            );
            let result = subgame.play();
            match result {
                None => panic!("None result"),
                Some(Player2) => (Player2, vec![p2, p1]),
                Some(Player1) => (Player1, vec![p1, p2]),
            }
        } else {
            if p1 > p2 {
                (Player1, vec![p1, p2])
            } else {
                (Player2, vec![p2, p1])
            }
        };

        let p = if let Player1 = player {
            &mut self.player1
        } else {
            &mut self.player2
        };

        for &card in &cards {
            p.push_back(card)
        }

        Ok(player)
    }

    fn score(&self) -> Option<usize> {
        let winner = if self.player2.is_empty() {
            &self.player1
        } else if self.player1.is_empty() {
            &self.player2
        } else {
            return None;
        };

        Some(
            winner
                .iter()
                .rev()
                .enumerate()
                .map(|(i, val)| (i + 1) * val)
                .sum(),
        )
    }

    fn play(&mut self) -> Option<Player> {
        let mut round = self.next_round().ok()?;
        while let Ok(r) = self.next_round() {
            round = r
        }
        Some(round)
    }
}

#[allow(unused_must_use)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\
                     \n\
                     Player 2:\n5\n8\n4\n7\n10";

        let game: Game = input.parse().unwrap();
        assert_eq!(game.player1, VecDeque::from(vec![9, 2, 6, 3, 1]));
        assert_eq!(game.player2, VecDeque::from(vec![5, 8, 4, 7, 10]));
    }

    #[test]
    fn test_play() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\
                     \n\
                     Player 2:\n5\n8\n4\n7\n10";

        let mut game: Game = input.parse().unwrap();
        game.next_round();

        assert_eq!(game.player1, VecDeque::from(vec![2, 6, 3, 1, 9, 5]));
        assert_eq!(game.player2, VecDeque::from(vec![8, 4, 7, 10]));

        assert_eq!(game.play(), Some(Player::Player2));
        assert_eq!(game.round, 17);
        assert_eq!(game.score(), Some(291));

        let mut subgame = Game::new(
            VecDeque::from(vec![9, 8, 5, 2]),
            VecDeque::from(vec![10, 1, 7]),
        );

        assert_eq!(subgame.play(), Some(Player::Player2));
        assert_eq!(subgame.round, 6);
    }

    #[test]
    fn test_no_infinite() {
        let input = "Player 1:\n\
                     43\n\
                     19\n\
                     \n\
                     Player 2:\n\
                     2\n\
                     29\n\
                     14";
        let mut game: Game = input.parse().unwrap();
        for _ in 0..10 {
            let _ = game.next_round();
        }

        println!("Result: {:?}", game.play());
        println!("Game: {:?}", game);
        assert_eq!(game.player2, VecDeque::from(vec![]));
    }
}

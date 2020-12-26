use std::collections::VecDeque;
use std::str::FromStr;

fn main() {
    let input = "167248359";
    let mut game: Game = input.parse().unwrap();
    for _ in 1..=100 {
        game.perform();
    }
    println!("{}", game.circle());
}

struct Game {
    cups: VecDeque<usize>,
}

impl FromStr for Game {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cups = s
            .chars()
            .map(|c| c.to_string().parse::<usize>().map_err(|_| "bad number"))
            .collect::<Result<VecDeque<usize>, _>>()?;
        Ok(Game { cups })
    }
}

impl Game {
    fn perform(&mut self) {
        let mut circle = self.cups.clone();
        let len = circle.len();
        let decrement = move |c: usize| {
            if c == 1 {
                len
            } else {
                c - 1
            }
        };
        let cup = circle.front().unwrap().clone();
        let mut target = cup;
        circle.rotate_left(1);
        let mut rest = circle.split_off(3);
        let mut found = None;

        while let None = found {
            target = decrement(target);
            found = rest
                .iter()
                .enumerate()
                .find(|(_, &c)| c == target)
                .map(|(i, _)| i);
        }

        let destination = found.unwrap().clone();
        let mut before = rest.split_off(destination + 1);
        rest.append(&mut circle);
        rest.append(&mut before);
        self.cups = rest;
    }

    fn circle(&self) -> String {
        let mut circle = self.cups.clone();
        for _ in 0..9 {
            if let Some(1) = circle.front() {
                break;
            }
            circle.rotate_left(1);
        }
        circle.iter().skip(1).map(|i| i.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "389125467";
        let mut game: Game = input.parse().unwrap();
        assert_eq!(game.cups, vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
        assert_eq!(game.circle(), "25467389");
        game.perform();
        assert_eq!(game.circle(), "54673289");
        game.perform();
        assert_eq!(game.circle(), "32546789");
        for _ in 3..=10 {
            game.perform();
        }
        assert_eq!(game.circle(), "92658374");
        for _ in 11..=100 {
            game.perform();
        }
        assert_eq!(game.circle(), "67384529");
    }
}

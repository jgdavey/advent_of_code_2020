use std::collections::HashMap;
use std::str::FromStr;

fn string_to_vec(s: &str) -> Result<Vec<usize>, &'static str> {
    s.chars()
        .map(|c| c.to_string().parse::<usize>().map_err(|_| "bad number"))
        .collect::<Result<Vec<usize>, _>>()
}

fn main() {
    let input = "167248359";
    let mut game = Game::new(string_to_vec(input).unwrap(), 9);
    for _ in 1..=100 {
        game.perform();
    }
    let next = game.next(&1, 9);
    println!("{:?} => product {}", next, next.iter().product::<usize>());

    let mut game = Game::new(string_to_vec(input).unwrap(), 1_000_000);
    for _ in 1..=10_000_000 {
        game.perform();
    }

    let next = game.next(&1, 2);
    println!("{:?} => product {}", next, next.iter().product::<usize>());
}

struct Game {
    current: usize,
    cups: HashMap<usize, usize>,
}

impl FromStr for Game {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cups = s
            .chars()
            .map(|c| c.to_string().parse::<usize>().map_err(|_| "bad number"))
            .collect::<Result<Vec<usize>, _>>()?;
        Ok(Game::new(cups, 9))
    }
}

impl Game {
    fn new(initial: Vec<usize>, target_size: usize) -> Self {
        let current = initial.first().unwrap().clone();
        let mut cups = HashMap::with_capacity(target_size);
        for s in initial.windows(2) {
            cups.insert(s[0], s[1]);
        }
        let biggest = initial.iter().max().cloned().unwrap_or(0) + 1;
        if biggest <= target_size {
            cups.insert(*initial.last().unwrap(), biggest);
            for i in biggest..target_size {
                cups.insert(i, i + 1);
            }
            cups.insert(target_size, current.clone());
        } else {
            cups.insert(initial.last().unwrap().clone(), current);
        }
        Game { cups, current }
    }

    fn perform(&mut self) {
        let len = self.cups.len();
        let cup = self.current.clone();
        let next_three = self.next(&self.current, 3);

        let target = [1, 2, 3, 4]
            .iter()
            .map(move |d| if d >= &cup { len - (d - cup) } else { cup - d })
            .filter(|i| !next_three.contains(i))
            .next()
            .unwrap();

        let stitch = self.cups[&next_three[2]];

        // println!("current: {:?}", self.current);
        // println!("pick up: {:?}", next_three);
        // println!("destination: {:?}", target);
        // println!("stitch: {:?}", stitch);
        self.cups.insert(self.current.clone(), stitch);

        let insert_before = self.cups[&target];
        self.cups.insert(next_three[2], insert_before);
        self.cups.insert(target, next_three[0]);

        self.current = stitch;
    }

    fn next(&self, from: &usize, take: usize) -> Vec<usize> {
        let mut cursor = from.clone();
        let mut out = vec![];
        for _ in 0..take {
            cursor = self.cups[&cursor];
            out.push(cursor);
        }
        out
    }

    fn circle(&self) -> String {
        self.next(&self.current, self.cups.len())
            .iter()
            .map(|i| i.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "389125467";
        let mut game = Game::new(string_to_vec(input).unwrap(), 9);

        assert_eq!(game.circle(), "891254673");
        game.perform();
        game.perform();
        assert_eq!(game.circle(), "467891325");
        for _ in 3..=10 {
            game.perform();
        }
        assert_eq!(game.circle(), "374192658");

        let mut game = Game::new(string_to_vec(input).unwrap(), 1_000_000);

        for _ in 1..=10_000_000 {
            game.perform();
        }
        assert_eq!(game.next(&1, 2), vec![934001, 159792]);
        // In the above example (389125467), this would be 934001 and
        // then 159792; multiplying these together produces
        // 149245887792.
    }
}

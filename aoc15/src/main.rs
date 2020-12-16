use std::collections::HashMap;

fn main() {
    let input = &[0,14,6,20,1,4];
    let mut game = Game::new(input);
    game.go_to_turn(2020);
    println!("Turn: {}, Last spoken: {}", game.turn, game.last_spoken);

    game.go_to_turn(30_000_000);
    println!("Turn: {}, Last spoken: {}", game.turn, game.last_spoken);
}

#[derive(Debug, Default)]
struct Game {
    last_spoken: u32,
    turn: u32,
    spoken: HashMap<u32, u32>
}

impl Game {
    fn new(initial: &[u32]) -> Self {
        let mut game = Game::default();

        for &entry in initial {
            game.spoken.insert(entry, game.turn);
            game.turn += 1;
            game.last_spoken = entry;
        }

        game
    }

    fn tick(&mut self) -> u32 {
        let last_turn = self.turn - 1;
        let n = match self.spoken.get(&self.last_spoken) {
            Some(&e) => last_turn - e,
            _ => 0
        };
        self.spoken.insert(self.last_spoken, last_turn);
        self.last_spoken = n;
        self.turn += 1;
        self.last_spoken
    }

    fn go_to_turn(&mut self, turn: u32) -> u32 {
        for _ in 0..(turn - self.turn) {
            self.tick();
        }
        self.last_spoken
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turns() {
        let mut game = Game::new(&vec![0, 3, 6]);
        let mut history = vec![0, 3, 6];
        for _ in 3..10 {
            history.push(game.tick());
        }
        assert_eq!(history, vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0]);
    }

    #[test]
    fn test_part_1() {
        let mut game = Game::new(&vec![2, 1, 3]);
        assert_eq!(game.go_to_turn(2020), 10);

        let mut game = Game::new(&vec![1, 2, 3]);
        assert_eq!(game.go_to_turn(2020), 27);
    }
}

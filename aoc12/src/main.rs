use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    NorthSouth(isize),
    EastWest(isize),
    Rotate(usize),
    Move(isize),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Instruction, Self::Err> {
        use Instruction::*;

        let mut chars = s.chars();
        let inst = chars.next().ok_or("Invalid instruction")?;
        let num: isize = chars.collect::<String>().parse().map_err(|e| {
            println!("{}", e);
            "Invalid number"
        })?;
        match (inst, num) {
            ('N', n) => Ok(NorthSouth(n)),
            ('S', n) => Ok(NorthSouth(-n)),
            ('E', n) => Ok(EastWest(n)),
            ('W', n) => Ok(EastWest(-n)),
            ('F', n) => Ok(Move(n)),
            ('L', 90) | ('R', 270) => Ok(Rotate(1)),
            ('R', 90) | ('L', 270) => Ok(Rotate(3)),
            ('L', 180) | ('R', 180) => Ok(Rotate(2)),
            _ => Err("Nope."),
        }
    }
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Ship {
    position: Point,
    waypoint: Point,
}

/// The waypoint starts 10 units east and 1 unit north relative to the
/// ship. The waypoint is relative to the ship; that is, if the ship
/// moves, the waypoint moves with it.
impl Default for Ship {
    fn default() -> Self {
        Ship {
            position: Point { x: 0, y: 0 },
            waypoint: Point { x: 10, y: 1 },
        }
    }
}

impl Ship {
    fn apply_instruction(&mut self, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            NorthSouth(n) => self.waypoint.y += n,
            EastWest(n) => self.waypoint.x += n,
            Rotate(n) => {
                for _ in 0..*n {
                    let newy = self.waypoint.x;
                    self.waypoint.x = -self.waypoint.y;
                    self.waypoint.y = newy;
                }
            }
            Move(n) => {
                for _ in 0..*n {
                    self.position.x += self.waypoint.x;
                    self.position.y += self.waypoint.y;
                }
            }
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.position.x.abs() + self.position.y.abs()) as usize
    }
}

fn run(instructions: &[Instruction]) -> Ship {
    let mut ship = Ship::default();
    for inst in instructions {
        ship.apply_instruction(&inst);
    }
    ship
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let instructions: Vec<_> = input
        .lines()
        .map(|l| l.parse::<Instruction>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let ship = run(&instructions);
    println!("Ship: {:?} ({})", ship, ship.manhattan_distance());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let instructions: Vec<_> = "F10\n\
                                    N3\n\
                                    F7\n\
                                    R90\n\
                                    F11"
        .lines()
        .map(|l| l.parse::<Instruction>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
        let ship = run(&instructions);
        assert_eq!(ship.position.x, 214);
        assert_eq!(ship.position.y, -72);
    }
}

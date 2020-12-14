use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn rotate(&self, times: usize) -> Direction {
        let mapping = match self {
            Direction::East => 0,
            Direction::North => 1,
            Direction::West => 2,
            Direction::South => 3
        };
        match (mapping + times) & 3 {
            0 => Direction::East,
            1 => Direction::North,
            2 => Direction::West,
            3 => Direction::South,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
enum Instruction {
    NorthSouth(isize),
    EastWest(isize),
    Rotate(isize),
    Move(isize)
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Instruction, Self::Err> {
        use Instruction::*;

        let mut chars = s.chars();
        let inst = chars.next().ok_or("Invalid instruction")?;
        let num: isize = chars.collect::<String>().parse().map_err(|e| {println!("{}", e); "Invalid number"})?;
        match (inst, num) {
            ('N', n) => Ok(NorthSouth(-n)),
            ('S', n) => Ok(NorthSouth(n)),
            ('E', n) => Ok(EastWest(n)),
            ('W', n) => Ok(EastWest(-n)),
            ('F', n) => Ok(Move(n)),
            ('L', 90) | ('R', 270) => Ok(Rotate(1)),
            ('R', 90) | ('L', 270) => Ok(Rotate(3)),
            ('L', 180) | ('R', 180) => Ok(Rotate(2)),
            _ => Err("Nope.")
        }
    }
}

#[derive(Debug)]
struct Ship {
    x: isize,
    y: isize,
    facing: Direction
}

impl Ship {
    fn apply_instruction(&mut self, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            NorthSouth(n) => self.y += n,
            EastWest(n) => self.x += n,
            Rotate(n) => self.facing = self.facing.rotate(*n as usize),
            Move(n) => match self.facing {
                Direction::North => self.y -= n,
                Direction::South => self.y += n,
                Direction::West  => self.x -= n,
                Direction::East  => self.x += n
            }
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

fn run(instructions: &[Instruction]) -> Ship {
    let mut ship = Ship { x: 0, y: 0, facing: Direction::East };
    for inst in instructions {
        ship.apply_instruction(&inst);
    }
    ship
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let instructions: Vec<_> = input.lines().map(|l| l.parse::<Instruction>()).collect::<Result<Vec<_>,_>>().unwrap();

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
                                    F11".lines().map(|l| l.parse::<Instruction>()).collect::<Result<Vec<_>,_>>().unwrap();
        let ship = run(&instructions);
        assert_eq!(ship.x, 17);
        assert_eq!(ship.y, 8);
    }
}

// These instructions would be handled as follows:

// F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
// N3 would move the ship 3 units north to east 10, north 3.
// F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
// R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
// F11 would move the ship 11 units south to east 17, south 8.
// At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of its east/west position and its north/south position) from its starting position is 17 + 8 = 25.

// Figure out where the navigation instructions lead. What is the Manhattan distance between that location and the ship's starting position?

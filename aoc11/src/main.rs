use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Space {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Space {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Space::Floor,
            'L' => Space::EmptySeat,
            '#' => Space::OccupiedSeat,
            _ => unimplemented!("yikes"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Space::Floor => '.',
            Space::EmptySeat => 'L',
            Space::OccupiedSeat => '#',
        }
    }

    fn is_occupied(&self) -> bool {
        matches!(self, Space::OccupiedSeat)
    }
}

#[derive(Default, Debug)]
struct Map {
    rows: usize,
    columns: usize,
    spaces: Vec<Space>,
}

impl Map {
    fn neighbors(rows: usize, cols: usize, idx: usize) -> Vec<usize> {
        let row = idx / cols;
        let col = idx % cols;

        let mut neighbors = vec![];

        if row != 0 {
            if col != 0 {
                neighbors.push(idx - cols - 1);
            }
            neighbors.push(idx - cols);
            if col != cols - 1 {
                neighbors.push(idx - cols + 1);
            }
        }

        if col != 0 {
            neighbors.push(idx - 1);
        }
        if col != cols - 1 {
            neighbors.push(idx + 1);
        }

        if row != (rows - 1) {
            if col != 0 {
                neighbors.push(idx + cols - 1);
            }
            neighbors.push(idx + cols);
            if col != cols - 1 {
                neighbors.push(idx + cols + 1);
            }
        }

        neighbors
    }

    fn tick(&mut self) -> bool {
        let mut new_spaces = Vec::with_capacity(self.spaces.len());
        let mut changed = false;
        for (i, space) in self.spaces.iter().enumerate() {
            let occupied_neighbors = Map::neighbors(self.rows, self.columns, i)
                .into_iter()
                .filter(|&idx| self.spaces[idx].is_occupied())
                .count();
            match (space, occupied_neighbors) {
                (Space::EmptySeat, 0) => {
                    changed = true;
                    new_spaces.push(Space::OccupiedSeat);
                }
                (Space::OccupiedSeat, x) if x >= 4 => {
                    changed = true;
                    new_spaces.push(Space::EmptySeat);
                }
                (space, _) => {
                    new_spaces.push(space.clone());
                }
            }
        }
        self.spaces = new_spaces;
        changed
    }

    fn stabilize(&mut self) {
        let mut okay = true;
        while okay {
            okay = self.tick();
        }
    }

    fn occupied_count(&self) -> usize {
        self.spaces
            .iter()
            .filter(|space| space.is_occupied())
            .count()
    }
}

impl FromStr for Map {
    type Err = &'static str;
    fn from_str(string: &str) -> Result<Map, Self::Err> {
        let input = string.trim();
        let rows = input.lines().count();
        if rows == 0 {
            return Ok(Default::default());
        }
        let columns = input.lines().next().unwrap().chars().count();
        let mut spaces = vec![];
        for line in input.lines() {
            if line.chars().count() != columns {
                return Err("Not all columns are the same width");
            }
            spaces.extend(line.chars().map(Space::from_char));
        }

        Ok(Map {
            rows,
            columns,
            spaces,
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, chunk) in self.spaces.chunks(self.columns).enumerate() {
            if i != 0 {
                writeln!(f)?;
            }
            for space in chunk {
                write!(f, "{}", space.to_char())?;
            }
        }
        Ok(())
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut map = input.parse::<Map>().unwrap();
    map.stabilize();
    println!("Occupied: {}", map.occupied_count());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_print() {
        let input = "L.LL.LL.LL\n\
                     LLLLL#L.LL\n\
                     L.L.L..L..\n\
                     LLLL.LL.LL\n\
                     L.#L.LL.LL\n\
                     L.LLLLL.LL\n\
                     ..L.L.....\n\
                     LLLLLLLLLL\n\
                     L.LLLLLL.L\n\
                     L.LLLLL.LL";
        let map = input.parse::<Map>().unwrap();
        assert_eq!(format!("{}", map), input);
    }

    #[test]
    fn test_neighbors() {
        //0  1  2  3
        //4  5  6  7
        //8  9 10 11
        assert_eq!(Map::neighbors(3, 4, 6), vec![1, 2, 3, 5, 7, 9, 10, 11]);
        assert_eq!(Map::neighbors(3, 4, 0), vec![1, 4, 5]);
        assert_eq!(Map::neighbors(3, 4, 7), vec![2, 3, 6, 10, 11]);
        assert_eq!(Map::neighbors(3, 4, 10), vec![5, 6, 7, 9, 11]);
    }

    #[test]
    fn test_ticks() {
        let input = "L.LL.LL.LL\n\
                     LLLLLLL.LL\n\
                     L.L.L..L..\n\
                     LLLL.LL.LL\n\
                     L.LL.LL.LL\n\
                     L.LLLLL.LL\n\
                     ..L.L.....\n\
                     LLLLLLLLLL\n\
                     L.LLLLLL.L\n\
                     L.LLLLL.LL";
        let mut map = input.parse::<Map>().unwrap();

        assert!(map.tick());
        assert_eq!(
            format!("{}", map),
            "#.##.##.##\n\
             #######.##\n\
             #.#.#..#..\n\
             ####.##.##\n\
             #.##.##.##\n\
             #.#####.##\n\
             ..#.#.....\n\
             ##########\n\
             #.######.#\n\
             #.#####.##"
        );

        assert!(map.tick());
        assert_eq!(
            format!("{}", map),
            "#.LL.L#.##\n\
             #LLLLLL.L#\n\
             L.L.L..L..\n\
             #LLL.LL.L#\n\
             #.LL.LL.LL\n\
             #.LLLL#.##\n\
             ..L.L.....\n\
             #LLLLLLLL#\n\
             #.LLLLLL.L\n\
             #.#LLLL.##"
        );
        assert!(map.tick());
        map.stabilize();

        let stabilized = "#.#L.L#.##\n\
                          #LLL#LL.L#\n\
                          L.#.L..#..\n\
                          #L##.##.L#\n\
                          #.#L.LL.LL\n\
                          #.#L#L#.##\n\
                          ..L.L.....\n\
                          #L#L##L#L#\n\
                          #.LLLLLL.L\n\
                          #.#L#L#.##";

        assert_eq!(format!("{}", map), stabilized);
        assert_eq!(map.tick(), false);
        assert_eq!(format!("{}", map), stabilized);
        assert_eq!(map.occupied_count(), 37);
    }
}

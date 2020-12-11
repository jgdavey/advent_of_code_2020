use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct TickSettings {
    seats_only: bool,
    occupant_threshold: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn visible_seats(&self, idx: usize, seats_only: bool) -> Vec<Space> {
        let mut neighbors = vec![];
        let cols = self.columns as isize;
        let rows = self.rows as isize;
        let row = idx as isize / cols;
        let col = idx as isize % cols;

        let look = |(x, y), (dx, dy)| {
            let new = (x + dx, y + dy);
            if new.0 >= 0 && new.1 >= 0 && new.0 < rows && new.1 < cols {
                Some(new)
            } else {
                None
            }
        };

        for &dir in &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let mut cursor = (row, col);
            while let Some((x, y)) = look(cursor, dir) {
                let i = (x * cols + y) as usize;
                match (&self.spaces[i], seats_only) {
                    (seat, false)
                    | (seat @ Space::OccupiedSeat, true)
                    | (seat @ Space::EmptySeat, true) => {
                        neighbors.push(seat.clone());
                        break;
                    }
                    _ => (),
                }
                cursor = (x, y);
            }
        }
        neighbors
    }

    fn tick(&mut self, settings: &TickSettings) -> bool {
        let mut new_spaces = Vec::with_capacity(self.spaces.len());
        let mut changed = false;
        for (i, space) in self.spaces.iter().enumerate() {
            let occupied_neighbors = self
                .visible_seats(i, settings.seats_only)
                .iter()
                .filter(|space| space.is_occupied())
                .count();
            match (space, occupied_neighbors) {
                (Space::EmptySeat, 0) => {
                    changed = true;
                    new_spaces.push(Space::OccupiedSeat);
                }
                (Space::OccupiedSeat, x) if x >= settings.occupant_threshold => {
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

    fn stabilize(&mut self, settings: &TickSettings) {
        while self.tick(settings) {}
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

    let settings1 = TickSettings {
        seats_only: false,
        occupant_threshold: 4,
    };
    let mut map1 = input.parse::<Map>().unwrap();
    map1.stabilize(&settings1);
    println!("Part 1 occupied: {}", map1.occupied_count());

    let settings2 = TickSettings {
        seats_only: true,
        occupant_threshold: 5,
    };
    let mut map2 = input.parse::<Map>().unwrap();
    map2.stabilize(&settings2);
    println!("Part 2 occupied: {}", map2.occupied_count());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_neighbs(s: &str) -> Vec<Space> {
        s.chars().map(Space::from_char).collect()
    }

    fn to_neighbor_string(neighbors: &[Space]) -> String {
        neighbors.iter().map(|s| s.to_char()).collect()
    }

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
    fn test_visible_seats() {
        let input = "L.#.\n\
                     #..L\n\
                     L..#\n\
                     #.LL";

        let map = input.parse::<Map>().unwrap();
        assert_eq!(map.visible_seats(10, false), to_neighbs("..L.#.LL"));
        assert_eq!(map.visible_seats(0, false), to_neighbs(".#."));
        assert_eq!(map.visible_seats(0, true), to_neighbs("##L"));

        assert_eq!(map.visible_seats(15, false), to_neighbs(".#L"));
        assert_eq!(map.visible_seats(15, true), to_neighbs("L#L"));

        assert_eq!(map.visible_seats(7, false), to_neighbs("#...#"));
        assert_eq!(map.visible_seats(7, true), to_neighbs("###"));

        assert_eq!(map.visible_seats(12, false), to_neighbs("L.."));
        assert_eq!(map.visible_seats(12, true), to_neighbs("LL"));
    }

    #[test]
    fn test_visible_seats2() {
        let input = ".......#.\n\
                     ...#.....\n\
                     .#.......\n\
                     .........\n\
                     ..#L....#\n\
                     ....#....\n\
                     .........\n\
                     #..#.....";

        let map = input.parse::<Map>().unwrap();
        assert_eq!(map.spaces[39], Space::EmptySeat);
        assert_eq!(
            to_neighbor_string(&map.visible_seats(39, false)),
            "...#...#"
        );
        assert_eq!(to_neighbor_string(&map.visible_seats(39, true)), "########");
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
        let settings = TickSettings {
            seats_only: false,
            occupant_threshold: 4,
        };
        assert!(map.tick(&settings));
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

        println!("{}\n\n", map);
        assert!(map.tick(&settings));
        println!("{}\n\n", map);
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
        map.stabilize(&settings);

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
        assert_eq!(map.occupied_count(), 37);
    }

    #[test]
    fn test_part_2() {
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
        let settings = TickSettings {
            seats_only: true,
            occupant_threshold: 5,
        };
        map.stabilize(&settings);
        assert_eq!(map.occupied_count(), 26);
    }
}

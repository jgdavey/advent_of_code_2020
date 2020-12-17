use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Active,
    Inactive,
}

impl Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Active => '#',
            Cell::Inactive => '.',
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_char())
    }
}

type Coordinate = (isize, isize, isize);

struct Grid3d {
    cells: HashMap<Coordinate, Cell>,
}

impl FromStr for Grid3d {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let cell = match c {
                    '.' => Cell::Inactive,
                    '#' => Cell::Active,
                    _ => return Err("Invalid character in grid"),
                };
                // initial z is always 0
                cells.insert((0, y as isize, x as isize), cell);
            }
        }
        Ok(Grid3d { cells })
    }
}

fn min_max_bounds<T: Ord + Default + Copy>(mut v: Vec<T>) -> RangeInclusive<T> {
    if v.is_empty() {
        return RangeInclusive::new(Default::default(), Default::default()).into();
    }
    v.sort_unstable();
    let min = v[0];
    let max = v[v.len() - 1];
    RangeInclusive::new(min, max)
}

fn neighbors((z, y, x): Coordinate) -> Vec<Coordinate> {
    lazy_static! {
        static ref OFFSETS: Vec<Coordinate> = {
            let mut offsets = vec![];
            for z in -1..=1 {
                for y in -1..=1 {
                    for x in -1..=1 {
                        let coord = (z, y, x);
                        if coord != (0, 0, 0) {
                            offsets.push(coord);
                        }
                    }
                }
            }
            offsets
        };
    }

    OFFSETS
        .iter()
        .map(|(dz, dy, dx)| (z + dz, y + dy, x + dx))
        .collect()
}

impl Grid3d {
    fn bounds(
        &self,
    ) -> (
        RangeInclusive<isize>,
        RangeInclusive<isize>,
        RangeInclusive<isize>,
    ) {
        let mut zs = vec![];
        let mut ys = vec![];
        let mut xs = vec![];
        for &(z, y, x) in self.cells.keys() {
            zs.push(z);
            ys.push(y);
            xs.push(x);
        }
        (min_max_bounds(zs), min_max_bounds(ys), min_max_bounds(xs))
    }

    fn to_string(&self, z: isize) -> String {
        let (_, ys, xs) = self.bounds();
        let out = ys
            .map(|y| {
                xs.clone()
                    .map(|x| {
                        self.cell_at((z, y, x)).to_char()
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        out
    }

    fn cell_at(&self, coord: Coordinate) -> Cell {
        *self.cells.get(&coord).unwrap_or(&Cell::Inactive)
    }

    fn active_count(&self) -> usize {
        self.cells.values().filter(|cell| matches!(cell, Cell::Active)).count()
    }

    fn tick(&mut self) {
        let (bz, by, bx) = self.bounds();

        let outer_cells = self.cells.keys().filter(|&(z, y, x)| {
            z == bz.start() || z == bz.end() || y == by.start() || y == by.end() || x == bx.start() || x == bx.end()
        }).cloned().collect::<Vec<Coordinate>>();


        // grow the cube by one layer
        for coord in outer_cells {
            for neighbor in neighbors(coord) {
                self.cells.entry(neighbor).or_insert_with(|| Cell::Inactive);
            }
        }

        let active_neighbors = self
            .cells
            .keys()
            .map(|&coord| {
                let active = neighbors(coord)
                    .iter()
                    .filter_map(|coord| self.cells.get(coord))
                    .filter(|cell| matches!(cell, Cell::Active))
                    .count();
                (coord, active)
            })
            .collect::<HashMap<_, _>>();

        for (coord, cell) in self.cells.iter_mut() {
            let active = *active_neighbors.get(coord).unwrap_or(&0);
            match (&cell, active) {
                (Cell::Inactive, 3) => *cell = Cell::Active,
                (Cell::Active, n) if n < 2 || n > 3 => *cell = Cell::Inactive,
                _ => (),
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut grid: Grid3d = input.parse().unwrap();

    for i in 1..=6 {
        grid.tick();
        println!("After round {}, {} are active", i, grid.active_count());
    }

    println!("z=0\n{}", grid.to_string(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid() {
        let input = ".#.\n\
                     ..#\n\
                     ###";
        let grid: Grid3d = input.parse().unwrap();
        assert_eq!(grid.bounds(), (0..=0, 0..=2, 0..=2));
        assert_eq!(grid.to_string(0), input);
    }

    #[test]
    fn test_tick() {
        let input = ".#.\n\
                     ..#\n\
                     ###";
        let mut grid: Grid3d = input.parse().unwrap();

        assert_eq!(grid.cell_at((0, 0, 0)), Cell::Inactive);
        assert_eq!(grid.cell_at((0, 0, 1)), Cell::Active);
        assert_eq!(grid.cell_at((0, 0, 2)), Cell::Inactive);

        assert_eq!(grid.cell_at((0, 1, 0)), Cell::Inactive);
        assert_eq!(grid.cell_at((0, 1, 1)), Cell::Inactive);
        assert_eq!(grid.cell_at((0, 1, 2)), Cell::Active);

        assert_eq!(grid.cell_at((0, 2, 0)), Cell::Active);
        assert_eq!(grid.cell_at((0, 2, 1)), Cell::Active);
        assert_eq!(grid.cell_at((0, 2, 2)), Cell::Active);

        // If a cube is active and exactly 2 or 3 of its neighbors are
        // also active, the cube remains active. Otherwise, the cube
        // becomes inactive.

        // If a cube is inactive but exactly 3 of
        // its neighbors are active, the cube becomes active.
        // Otherwise, the cube remains inactive.

        grid.tick();

        assert_eq!(grid.cell_at((0, 0, 0)), Cell::Inactive);
        assert_eq!(grid.cell_at((0, 0, 1)), Cell::Inactive);
        assert_eq!(grid.cell_at((0, 0, 2)), Cell::Inactive);

        assert_eq!(grid.cell_at((0, 1, 0)), Cell::Active);
        assert_eq!(grid.cell_at((0, 1, 1)), Cell::Inactive);
        assert_eq!(grid.cell_at((0, 1, 2)), Cell::Active);

        assert_eq!(grid.cell_at((0, 2, 0)), Cell::Inactive);
        assert_eq!(grid.cell_at((0, 2, 1)), Cell::Active);
        assert_eq!(grid.cell_at((0, 2, 2)), Cell::Active);

        for _ in 1..6 {
            grid.tick();
        }
        assert_eq!(grid.active_count(), 112);
    }
}

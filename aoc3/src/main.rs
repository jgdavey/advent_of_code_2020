use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SquareParseError;

type SquareResult<T> = Result<T, SquareParseError>;

impl fmt::Display for SquareParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SquareParseError")
    }
}

impl std::error::Error for SquareParseError {
    #[inline]
    fn description(&self) -> &str {
        "Unable to parse"
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Open,
    Tree,
}

impl FromStr for Square {
    type Err = SquareParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Square::Tree),
            "." => Ok(Square::Open),
            _ => Err(SquareParseError {}),
        }
    }
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<Square>>,
}

impl FromStr for Grid {
    type Err = SquareParseError;
    fn from_str(s: &str) -> SquareResult<Self> {
        let mut rows = vec![];
        for line in s.lines() {
            let mut row = vec![];
            for c in line.matches(|_| true) {
                row.push(c.parse()?);
            }
            rows.push(row);
        }
        Ok(Grid { rows })
    }
}

impl Grid {
    fn traverse(&self, right: usize, down: usize) -> Vec<Square> {
        self.rows
            .iter()
            .step_by(down)
            .enumerate()
            .map(|(i, row)| {
                let col = (i * right) % row.len();
                row[col]
            })
            .collect()
    }

    fn ouches(&self, right: usize, down: usize) -> usize {
        self.traverse(right, down)
            .into_iter()
            .filter(|&square| square == Square::Tree)
            .count()
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = input.parse::<Grid>().unwrap();
    let d1r1 = grid.ouches(1, 1);
    let d1r3 = grid.ouches(3, 1);
    let d1r5 = grid.ouches(5, 1);
    let d1r7 = grid.ouches(7, 1);
    let d2r1 = grid.ouches(1, 2);
    println!("Down 1, Right 1: {}", d1r1);
    println!("Down 1, Right 3: {}", d1r3);
    println!("Down 1, Right 5: {}", d1r5);
    println!("Down 1, Right 7: {}", d1r7);
    println!("Down 2, Right 1: {}", d2r1);
    println!("Product: {}", d1r1 * d1r3 * d1r5 * d1r7 * d2r1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_GRID: &str = "..##.......\n\
                                #...#...#..\n\
                                .#....#..#.\n\
                                ..#.#...#.#\n\
                                .#...##..#.\n\
                                ..#.##.....\n\
                                .#.#.#....#\n\
                                .#........#\n\
                                #.##...#...\n\
                                #...##....#\n\
                                .#..#...#.#";

    #[test]
    fn test_parse_grid() {
        let grid: Grid = EXAMPLE_GRID.parse().unwrap();
        assert_eq!(grid.rows.len(), 11);
        assert_eq!(grid.rows[0][0], Square::Open);
        assert_eq!(grid.rows[0][2], Square::Tree);
    }

    #[test]
    fn test_traversal() {
        use Square::*;
        let grid: Grid = EXAMPLE_GRID.parse().unwrap();
        assert_eq!(
            grid.traverse(3, 1),
            &[Open, Open, Tree, Open, Tree, Tree, Open, Tree, Tree, Tree, Tree]
        );

        assert_eq!(grid.ouches(3, 1), 7);
        assert_eq!(grid.ouches(1, 1), 2);
        assert_eq!(grid.ouches(1, 2), 2);
    }
}

use std::collections::HashSet;
use std::str::{Chars, FromStr};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let pattern: Pattern = input.parse().unwrap();
    let floor = pattern.floor();
    println!("Flipped: {}", floor.flipped_count());
}

struct Pattern {
    paths: Vec<Path>,
}

impl Pattern {
    fn floor(&self) -> Floor {
        let mut flipped = HashSet::new();
        for coord in self.paths.iter().map(|p| p.coordinate()) {
            if flipped.contains(&coord) {
                flipped.remove(&coord);
            } else {
                flipped.insert(coord);
            }
        }
        Floor { flipped }
    }
}

// r: row, identical a standard "y" value, axis like |
// q: "column", on a slant, axis like /
// q, r
type Coord = (isize, isize);

struct Path(Vec<Coord>);

impl Path {
    fn coordinate(&self) -> Coord {
        self.0.iter().fold((0,0), |(q, r), (dq, dr)| (q + dq, r + dr))
    }
}

struct CoordIter<'a> {
    chars: Chars<'a>,
}

impl<'a> Iterator for CoordIter<'a> {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        let coord = match self.chars.next()? {
            'e' => (1, 0),
            'w' => (-1, 0),
            c => {
                let n = self.chars.next()?;
                match (c, n) {
                    ('n', 'w') => (0, -1),
                    ('n', 'e') => (1, -1),
                    ('s', 'e') => (0, 1),
                    ('s', 'w') => (-1, 1),
                    _ => return None,
                }
            }
        };
        Some(coord)
    }
}

impl FromStr for Pattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let paths = s
            .lines()
            .map(|l| Path(CoordIter { chars: l.chars() }.collect::<Vec<Coord>>()))
            .collect();
        Ok(Pattern { paths })
    }
}

struct Floor {
    flipped: HashSet<Coord>
}

impl Floor {
    fn flipped_count(&self) -> usize {
        self.flipped.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw\n\
                     neeenesenwnwwswnenewnwwsewnenwseswesw\n\
                     seswneswswsenwwnwse\n\
                     nwnwneseeswswnenewneswwnewseswneseene\n\
                     swweswneswnenwsewnwneneseenw\n\
                     eesenwseswswnenwswnwnwsewwnwsene\n\
                     sewnenenenesenwsewnenwwwse\n\
                     wenwwweseeeweswwwnwwe\n\
                     wsweesenenewnwwnwsenewsenwwsesesenwne\n\
                     neeswseenwwswnwswswnw\n\
                     nenwswwsewswnenenewsenwsenwnesesenew\n\
                     enewnwewneswsewnwswenweswnenwsenwsw\n\
                     sweneswneswneneenwnewenewwneswswnese\n\
                     swwesenesewenwneswnwwneseswwne\n\
                     enesenwswwswneneswsenwnewswseenwsese\n\
                     wnwnesenesenenwwnenwsewesewsesesew\n\
                     nenewswnwewswnenesenwnesewesw\n\
                     eneswnwswnwsenenwnwnwwseeswneewsenese\n\
                     neswnwewnwnwseenwseesewsenwsweewe\n\
                     wseweeenwnesenwwwswnew";
        let pattern: Pattern = input.parse().unwrap();
        assert_eq!(pattern.paths[2].coordinate(), (-3, 3));
        assert_eq!(pattern.floor().flipped_count(), 10);
    }
}

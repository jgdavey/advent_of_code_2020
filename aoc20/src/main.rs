use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

const MASK: u16 = 0b1111111111;
const MONSTER: &str = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ";
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let tiles: Vec<Tile> = parse_tiles(&input);
    let solved = solve(&tiles);
    let mut art = assemble(solved.as_slice());
    let monster_tiles = art.find_monsters();
    let roughness = art.roughness();
    println!("{}", art.to_string(&monster_tiles, 'O'));
    println!("Roughness: {}", roughness);
}
type SidePattern = u16;

fn string_to_side(s: &str) -> SidePattern {
    let mut id = 0;
    for c in s.chars() {
        id = id << 1;
        match c {
            '#' => id |= 1,
            '.' => (),
            _ => panic!("Illegal char"),
        }
    }
    id
}

fn invert(n: SidePattern) -> SidePattern {
    (n.reverse_bits() >> 6) & MASK
}

#[derive(Clone, Debug)]
struct Image {
    rows: Vec<Vec<char>>,
}

impl Image {
    fn flip_x(&mut self) {
        for line in self.rows.iter_mut() {
            line.reverse();
        }
    }

    //[[a, b, c],  ->  [[g, d, a],
    // [d, e, f],  ->   [h, e, b],
    // [g, h, i]]  ->   [i, f, c]]
    fn rotate(&mut self) {
        let max = self.rows.len() - 1;
        let mut image = self.rows.clone();
        for x in 0..=max {
            for y in 0..=max {
                image[x][max - y] = self.rows[y][x]
            }
        }
        self.rows = image;
    }

    fn monster_offsets() -> Vec<(usize, usize)> {
        MONSTER
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((y, x), c)))
            .filter_map(|(coord, c)| if c == '#' { Some(coord) } else { None })
            .collect()
    }

    fn find_monsters(&mut self) -> Vec<(usize, usize)> {
        let m = Image::monster_offsets();
        let ymax = self.rows.len() - MONSTER.lines().count() + 1;
        let xmax = self.rows[0].len() - MONSTER.lines().next().unwrap().len() + 1;
        let mut monsters = vec![];
        for _ in 0..=1 {
            for _ in 0..4 {
                for y in 0..=ymax {
                    for x in 0..=xmax {
                        if m.iter().all(|(dy, dx)| self.rows[y + dy][x + dx] == '#') {
                            monsters.push((y, x));
                        }
                    }
                }
                if !monsters.is_empty() {
                    break
                }
                self.rotate();
            }
            self.flip_x();
        }

        monsters.iter().flat_map(|(y, x)| m.iter().map(move |(dy, dx)| (y + dy, x + dx))).collect()
    }

    fn roughness(&mut self) -> usize {
        let monsters = self.find_monsters();
        let monster_marks = monsters.len();
        let total_marks: usize = self.rows.iter().map(|row| row.iter().filter(|&c| *c == '#').count()).sum();
        println!("Monster marks: {}, Total marks: {}", monster_marks, total_marks);
        total_marks - monster_marks
    }

    fn to_string(&self, replace_coords: &[(usize, usize)], replace_char: char) -> String {
        let mut rows = self.rows.clone();
        for &(y, x) in replace_coords {
            rows[y][x] = replace_char;
        }
        let art = rows
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        art

    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string(&[], '#'))
    }
}

#[derive(Clone, Debug)]
struct Tile {
    id: usize,
    sides: [SidePattern; 4],
    image: Image,
}

impl Tile {
    fn possible_sides(&self) -> Vec<SidePattern> {
        let mut set = Vec::with_capacity(8);
        for &side in self.sides.iter() {
            set.push(side);
            set.push(invert(side));
        }
        set
    }

    fn flip_x(&mut self) {
        self.sides = [
            invert(self.sides[0]),
            invert(self.sides[3]),
            invert(self.sides[2]),
            invert(self.sides[1]),
        ];
        self.image.flip_x();
    }

    //[[a, b, c],  ->  [[g, d, a],
    // [d, e, f],  ->   [h, e, b],
    // [g, h, i]]  ->   [i, f, c]]
    fn rotate(&mut self) {
        self.sides = [
            self.sides[3],
            self.sides[0],
            self.sides[1],
            self.sides[2],
        ];
        self.image.rotate();
    }

    fn orient_to(&mut self, target: SidePattern, side: usize) {
        for _ in 0..2 {
            if self.sides[side] == target {
                return;
            }
            if let Some(n) = self
                .sides
                .iter()
                .enumerate()
                .find(|&(_, s)| *s == target)
                .map(|(n, _)| n)
            {
                // side = 3
                // n = 1

                // side = 1
                // n = 207
                for _ in 0..((side + 4 - n) % 4) {
                    self.rotate();
                }
                return;
            } else {
                self.flip_x();
            }
        }
        assert_eq!(self.sides[side], target);
    }
}

impl FromStr for Tile {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        let line1 = lines.next().unwrap();
        let id = line1[5..line1.find(':').unwrap()].parse().unwrap();
        let rest = lines.collect::<Vec<_>>();
        let top = rest[0].to_string();
        let left = rest
            .iter()
            .map(|l| l.chars().next().unwrap())
            .rev()
            .collect::<String>();
        let right = rest
            .iter()
            .map(|l| l.chars().last().unwrap())
            .collect::<String>();
        let bottom = rest.last().unwrap().chars().rev().collect::<String>();
        let image_rows = rest[1..rest.len() - 1]
            .iter()
            .map(|line| {
                line[1..line.len() - 1]
                    .to_string()
                    .chars()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let image = Image { rows: image_rows };
        // clockwise from top
        let sides = [
            string_to_side(&top),
            string_to_side(&right),
            string_to_side(&bottom),
            string_to_side(&left),
        ];
        Ok(Tile { id, sides, image })
    }
}

fn parse_tiles(s: &str) -> Vec<Tile> {
    let cap = s.lines().count() / 12;
    let mut out = Vec::with_capacity(cap);
    for l in s.trim().split("\n\n") {
        out.push(l.parse().unwrap())
    }
    out
}

fn solve(tiles: &[Tile]) -> Vec<Vec<Tile>> {
    let tilemap: HashMap<usize, Tile> = tiles.iter().map(|t| (t.id, t.clone())).collect();
    let mut possibles: HashMap<SidePattern, HashSet<usize>> = HashMap::new();

    for tile in tiles {
        for side in tile.possible_sides() {
            let entry = possibles.entry(side).or_insert_with(HashSet::new);
            entry.insert(tile.id);
        }
    }

    let neighbors = tiles
        .iter()
        .map(|tile| {
            let mut neigh = HashSet::new();
            for side in tile.possible_sides() {
                if let Some(ts) = possibles.get(&side) {
                    for &t in ts {
                        if t != tile.id {
                            neigh.insert(t);
                        }
                    }
                }
            }
            (tile.id, neigh)
        })
        .collect::<HashMap<usize, HashSet<usize>>>();
    // find corners

    let mut corners = Vec::with_capacity(4);
    for (tile_id, n) in neighbors {
        match n.len() {
            2 => corners.push(tile_id),
            3 =>
            /* border */
            {
                ()
            }
            4 =>
            /* middle */
            {
                ()
            }
            _ => panic!("Too many neighbors"),
        }
    }

    let side_length = (tiles.len() as f64).sqrt() as usize;
    // let mut grid = Vec::with_capacity(side_length);

    let mut start = tilemap.get(&corners[0]).unwrap().clone();
    match (
        possibles.get(&start.sides[1]).map(|h| h.len()),
        possibles.get(&start.sides[2]).map(|h| h.len()),
    ) {
        (Some(2), Some(2)) => (),
        (Some(1), Some(2)) => {
            start.rotate();
            start.rotate();
            start.rotate()
        }
        (Some(1), Some(1)) => {
            start.rotate();
            start.rotate()
        }
        (Some(2), Some(1)) => start.rotate(),
        _ => panic!("What"),
    }

    match (
        possibles.get(&start.sides[1]).map(|h| h.len()),
        possibles.get(&start.sides[2]).map(|h| h.len()),
    ) {
        (Some(2), Some(2)) => (),
        _ => panic!("What"),
    }

    let mut rows: Vec<Vec<Tile>> = vec![];
    let mut row = vec![start];
    loop {
        while row.len() < side_length {
            let last = &row[row.len() - 1];
            let target = invert(last.sides[1]);
            let mut neigh = possibles
                .get(&target)
                .unwrap()
                .iter()
                .find(|&t| last.id != *t)
                .and_then(|id| tilemap.get(id))
                .cloned()
                .unwrap();
            neigh.orient_to(target, 3);
            row.push(neigh);
        }
        let starter = row[0].clone();
        rows.push(row);
        let bottom = invert(starter.sides[2]);

        if let Some(mut tile) = possibles
            .get(&bottom)
            .and_then(|h| h.iter().find(|&t| starter.id != *t))
            .and_then(|id| tilemap.get(id).cloned())
        {
            tile.orient_to(bottom, 0);
            if let Some(1) = possibles.get(&tile.sides[1]).map(|h| h.len()) {
                tile.flip_x();
            }
            row = vec![tile];
        } else {
            break;
        }
    }
    rows
}

fn assemble(solution: &[Vec<Tile>]) -> Image {
    let t0 = &solution[0][0];
    let inner_width = t0.image.rows.len();
    let mut rows: Vec<Vec<char>> = vec![];
    for row in solution {
        for y in 0..inner_width {
            let mut out = vec![];
            for tile in row {
                for &c in &tile.image.rows[y] {
                    out.push(c);
                }
            }
            rows.push(out);
        }
    }
    Image { rows }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tile() {
        let input = "Tile 2311:\n\
                     ..##.#..#.\n\
                     ##..#.....\n\
                     #...##..#.\n\
                     ####.#...#\n\
                     ##.##.###.\n\
                     ##...#.###\n\
                     .#.#.#..##\n\
                     ..#....#..\n\
                     ###...#.#.\n\
                     ..###..###";
        let tile: Tile = input.parse().unwrap();
        assert_eq!(tile.id, 2311);
        assert_eq!(
            tile.sides,
            [
                0b_00110_10010,
                0b_00010_11001,
                0b_11100_11100,
                0b_01001_11110
            ]
        );

        let pattern = "#..#....\n\
                       ...##..#\n\
                       ###.#...\n\
                       #.##.###\n\
                       #...#.##\n\
                       #.#.#..#\n\
                       .#....#.\n\
                       ##...#.#";
        let image_rows = pattern
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        assert_eq!(tile.image.rows, image_rows);
    }

    #[test]
    fn test_part_1() {
        let input = "Tile 2311:\n\
                     ..##.#..#.\n\
                     ##..#.....\n\
                     #...##..#.\n\
                     ####.#...#\n\
                     ##.##.###.\n\
                     ##...#.###\n\
                     .#.#.#..##\n\
                     ..#....#..\n\
                     ###...#.#.\n\
                     ..###..###\n\
                     \n\
                     Tile 1951:\n\
                     #.##...##.\n\
                     #.####...#\n\
                     .....#..##\n\
                     #...######\n\
                     .##.#....#\n\
                     .###.#####\n\
                     ###.##.##.\n\
                     .###....#.\n\
                     ..#.#..#.#\n\
                     #...##.#..\n\
                     \n\
                     Tile 1171:\n\
                     ####...##.\n\
                     #..##.#..#\n\
                     ##.#..#.#.\n\
                     .###.####.\n\
                     ..###.####\n\
                     .##....##.\n\
                     .#...####.\n\
                     #.##.####.\n\
                     ####..#...\n\
                     .....##...\n\
                     \n\
                     Tile 1427:\n\
                     ###.##.#..\n\
                     .#..#.##..\n\
                     .#.##.#..#\n\
                     #.#.#.##.#\n\
                     ....#...##\n\
                     ...##..##.\n\
                     ...#.#####\n\
                     .#.####.#.\n\
                     ..#..###.#\n\
                     ..##.#..#.\n\
                     \n\
                     Tile 1489:\n\
                     ##.#.#....\n\
                     ..##...#..\n\
                     .##..##...\n\
                     ..#...#...\n\
                     #####...#.\n\
                     #..#.#.#.#\n\
                     ...#.#.#..\n\
                     ##.#...##.\n\
                     ..##.##.##\n\
                     ###.##.#..\n\
                     \n\
                     Tile 2473:\n\
                     #....####.\n\
                     #..#.##...\n\
                     #.##..#...\n\
                     ######.#.#\n\
                     .#...#.#.#\n\
                     .#########\n\
                     .###.#..#.\n\
                     ########.#\n\
                     ##...##.#.\n\
                     ..###.#.#.\n\
                     \n\
                     Tile 2971:\n\
                     ..#.#....#\n\
                     #...###...\n\
                     #.#.###...\n\
                     ##.##..#..\n\
                     .#####..##\n\
                     .#..####.#\n\
                     #..#.#..#.\n\
                     ..####.###\n\
                     ..#.#.###.\n\
                     ...#.#.#.#\n\
                     \n\
                     Tile 2729:\n\
                     ...#.#.#.#\n\
                     ####.#....\n\
                     ..#.#.....\n\
                     ....#..#.#\n\
                     .##..##.#.\n\
                     .#.####...\n\
                     ####.#.#..\n\
                     ##.####...\n\
                     ##..#.##..\n\
                     #.##...##.\n\
                     \n\
                     Tile 3079:\n\
                     #.#.#####.\n\
                     .#..######\n\
                     ..#.......\n\
                     ######....\n\
                     ####.#..#.\n\
                     .#...#.##.\n\
                     #.#####.##\n\
                     ..#.###...\n\
                     ..#.......\n\
                     ..#.###...";
        let _tiles: Vec<Tile> = parse_tiles(input);

        // let expected = vec![
        //     vec![1951, 2311, 3079],
        //     vec![2729, 1427, 2473],
        //     vec![2971, 1489, 1171],
        // ];

        // let solved = solve(&tiles).iter().map(|row| row.iter().map(|col| col.id).collect::<Vec<_>>()).collect::<Vec<_>>();

        // assert_eq!(solved, expected);
    }

    #[test]
    fn test_invert() {
        let a = 0b01000_00101;
        let b = 0b10100_00010;
        assert_eq!(invert(a), b);
        assert_eq!(invert(b), a);
        assert_eq!(invert(invert(b)), b);
        assert_eq!(invert(invert(a)), a);
    }

    #[test]
    fn test_find_monster() {
        let input = ".#.#..#.##...#.##..#####\n\
                     ###....#.#....#..#......\n\
                     ##.##.###.#.#..######...\n\
                     ###.#####...#.#####.#..#\n\
                     ##.#....#.##.####...#.##\n\
                     ...########.#....#####.#\n\
                     ....#..#...##..#.#.###..\n\
                     .####...#..#.....#......\n\
                     #..#.##..#..###.#.##....\n\
                     #.####..#.####.#.#.###..\n\
                     ###.#.#...#.######.#..##\n\
                     #.####....##..########.#\n\
                     ##..##.#...#...#.#.#.#..\n\
                     ...#..#..#.#.##..###.###\n\
                     .#.#....#.##.#...###.##.\n\
                     ###.#...#..#.##.######..\n\
                     .#.#.###.##.##.#..#.##..\n\
                     .####.###.#...###.#..#.#\n\
                     ..#.#..#..#.#.#.####.###\n\
                     #..####...#.#.#.###.###.\n\
                     #####..#####...###....##\n\
                     #.##..#..#...#..####...#\n\
                     .#.###..##..##..####.##.\n\
                     ...###...##...#...#..###";
        let image_rows = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut image = Image { rows: image_rows };

        // let m = Image::monster_offsets();
        // let x = 2;
        // let y = 2;
        // for (dy, dx) in m.iter() {
        //     println!("({}, {}): {}", (y + dy), (x + dx), image.rows[y + dy][x + dx]);
        // }
        //assert_eq!(image.find_monsters(), vec![(2, 2), (16, 1)]);

        assert_eq!(image.roughness(), 273);
    }

    #[test]
    fn test_rotate_image() {
        let input = "..#.\n\
                     #..#\n\
                     .###\n\
                     #...";
        let image_rows = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut image = Image { rows: image_rows };
        let actual = format!("{}", image);
        assert_eq!(actual, input);
        println!("A:\n{}", actual);
        image.rotate();

        let actual = format!("{}", image);
        println!("B:\n{}", actual);
        let expected = "#.#.\n\
                        .#..\n\
                        .#.#\n\
                        .##.";
        assert_eq!(actual, expected);
    }
}

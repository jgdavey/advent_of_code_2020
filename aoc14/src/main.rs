use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
struct Mask {
    ones: usize,
    float_initial: usize,
    float_masks: Vec<usize>,
}

impl FromStr for Mask {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ones: usize = 0;
        let mut float_initial: usize = 0;

        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '0' => continue,
                '1' => ones |= 1 << i,
                'X' => float_initial |= 1 << i,
                _ => return Err("Illegal pattern in mask"),
            }
        }

        Ok(Mask {
            ones,
            float_initial,
            float_masks: powerseti(float_initial),
        })
    }
}

fn powerseti(s: usize) -> Vec<usize> {
    let p = 2usize.pow(s.count_ones());
    let bits = (0..64)
        .map(|i| s & (1 << i))
        .filter(|&n| n > 0)
        .collect::<Vec<_>>();
    (0..p)
        .map(|i| {
            bits.iter()
                .enumerate()
                .filter(|&(idx, _)| (i >> idx) % 2 == 1)
                .fold(0, |acc, (_, bit)| acc | bit)
        })
        .collect()
}

impl Mask {
    fn apply(&self, other: usize) -> Vec<usize> {
        let start = (other | self.ones) & !self.float_initial;
        self.float_masks.iter().map(|mask| start | mask).collect()
    }
}

type Memory = BTreeMap<usize, usize>;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut mask = Mask::default();
    let mut mem = Memory::new();
    for line in input.lines() {
        let parts = line.split_whitespace().take(3).collect::<Vec<_>>();
        let inst = parts[0];
        if inst == "mask" {
            mask = parts[2].parse().unwrap();
        } else {
            let base = inst[4..inst.len() - 1].parse::<usize>().unwrap();
            let value = parts[2].parse::<usize>().unwrap();
            for address in mask.apply(base) {
                mem.insert(address, value);
            }
        }
    }
    println!("Memory: {:?}", mem.values().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_masks() {
        let input = "00000000000000000000000000000001X10X";
        let mask: Mask = input.parse().unwrap();
        assert_eq!(mask.ones, 20);
        assert_eq!(mask.float_initial, 0b01001);
        assert_eq!(mask.float_masks, vec![0b00000, 0b00001, 0b01000, 0b01001]);
    }

    #[test]
    fn test_apply_mask() {
        let float_initial = 0b100001;
        let mask = Mask {
            float_initial,
            float_masks: powerseti(float_initial),
            ones: 2 + 16,
        };
        assert_eq!(mask.apply(42), vec![26, 27, 58, 59]);
    }
}

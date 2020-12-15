use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Default)]
struct Mask {
    zeros: usize,
    ones: usize,
}

impl FromStr for Mask {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ones: usize = 0;
        let mut zeros: usize = 0;

        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '0' => zeros |= 1 << i,
                '1' => ones |= 1 << i,
                'X' => continue,
                _ => return Err("Illegal pattern in mask"),
            }
        }
        Ok(Mask { zeros, ones })
    }
}

impl Mask {
    fn apply(&self, other: usize) -> usize {
        (other | self.ones) & !self.zeros
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
            let address = inst[4..inst.len() - 1].parse::<usize>().unwrap();
            let value = parts[2].parse::<usize>().unwrap();
            mem.insert(address, mask.apply(value));
        }
    }
    println!("Memory: {:?}", mem.values().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_masks() {
        let input = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let mask: Mask = input.parse().unwrap();
        assert_eq!(mask.zeros, 2);
        assert_eq!(mask.ones, 64);
    }

    #[test]
    fn test_apply_mask() {
        let mask = Mask { zeros: 2, ones: 64 };
        assert_eq!(mask.apply(11), 73);
        assert_eq!(mask.apply(101), 101);
    }
}

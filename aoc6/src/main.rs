use std::collections::BTreeSet;

pub fn decode_group_union(group: &str) -> usize {
    let mut all = BTreeSet::new();
    for line in group.lines() {
        for c in line.chars() {
            all.insert(c);
        }
    }
    all.len()
}

pub fn decode_group(group: &str) -> usize {
    let mut sets = group
        .lines()
        .map(|line| line.chars().collect::<BTreeSet<char>>());
    if let Some(mut all) = sets.next() {
        for set in sets {
            all = all.intersection(&set).cloned().collect()
        }
        all.len()
    } else {
        0
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let groups: Vec<_> = input.trim().split("\n\n").map(decode_group).collect();
    println!("Sum: {}", groups.iter().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_group_union() {
        assert_eq!(decode_group_union("a\nb\nc"), 3);
        assert_eq!(decode_group_union("a\nba\nc"), 3);
        assert_eq!(decode_group_union("abc"), 3);
        assert_eq!(decode_group_union("a\nabc"), 3);
        assert_eq!(decode_group_union("a\nab\ncd"), 4);
    }

    #[test]
    fn test_decode_group() {
        assert_eq!(decode_group("abc"), 3);
        assert_eq!(decode_group("a\nb\nc"), 0);
        assert_eq!(decode_group("a\nabc"), 1);
    }
}

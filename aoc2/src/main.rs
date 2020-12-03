use std::ops::RangeInclusive;
use std::fs::read_to_string;

#[derive(Debug)]
struct Rule {
    char: char,
    rng: RangeInclusive<usize>
}

impl Rule {
    fn check(&self, input: &str) -> bool {
        let num = input.matches(self.char).count();
        self.rng.contains(&num)
    }
}

#[derive(Debug)]
struct Input {
    rule: Rule,
    password: String
}

impl Input {
    fn from_input(input: &str) -> Option<Self> {
        let mid = input.find(':')?;
        let (before, after) = input.split_at(mid);

        let mid = before.find(' ')?;
        let (range, c) = before.split_at(mid);

        let mut r = range.splitn(2, '-');
        let from: usize = r.next()?.parse().ok()?;
        let to: usize = r.next()?.parse().ok()?;
        Some(
            Input {
                rule: Rule {
                    char: c.trim().chars().next()?,
                    rng: from..=to
                },
                password: after[1..].trim().to_string()
            }
        )
    }

    fn is_valid(&self) -> bool {
        self.rule.check(&self.password)
    }
}

fn input_valid(input: &str) -> bool {
    if let Some(input) = Input::from_input(input) {
        input.is_valid()
    } else {
        false
    }
}

fn main() -> std::io::Result<()> {
    let inputs = read_to_string("input.txt")?;
    let valid_lines = inputs.lines().filter(|s| input_valid(s)).count();
    println!("Valid lines: {}", valid_lines);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_valid_construct() {
        let input = Input::from_input("1-3 a: abcde").unwrap();
        assert!(input.is_valid());
    }

    #[test]
    fn test_input_valid() {
        assert!(input_valid("2-9 c: ccccccccc"));
        assert!(!input_valid("2-8 c: ccccccccc"));
        assert!(!input_valid("1-3 b: cdefg"));
    }
}


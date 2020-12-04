use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::read_to_string;

type Error = &'static str;

mod validators {
    use super::Error;
    use std::collections::HashSet;
    use std::ops::RangeInclusive;
    type Result<'a> = std::result::Result<&'a str, Error>;

    pub fn in_range<'a>(input: &'a str, range: RangeInclusive<usize>) -> Result {
        let num = input.parse::<usize>().map_err(|_| "not a number")?;
        if range.contains(&num) {
            Ok(input)
        } else {
            Err("number not in range")
        }
    }

    pub fn one_of<'a>(input: &'a str, set: &[&str]) -> Result<'a> {
        let set: HashSet<&str> = set.iter().cloned().collect();
        if set.contains(input) {
            Ok(input)
        } else {
            Err("value not in allowed")
        }
    }

    pub fn is_color(input: &str) -> Result {
        if !input.starts_with('#') || !input.len() == 7 {
            return Err("colors must be # followed by 6 hex digits");
        }
        let without_prefix = input.trim_start_matches('#');
        if i64::from_str_radix(without_prefix, 16).is_ok() {
            Ok(input)
        } else {
            Err("invalid color number")
        }
    }

    pub fn valid_height(input: &str) -> Result {
        if input.ends_with("cm") {
            in_range(input.trim_end_matches("cm"), 150..=193)
        } else if input.ends_with("in") {
            in_range(input.trim_end_matches("in"), 59..=76)
        } else {
            Err("height must end with in or cm")
        }
    }

    pub fn number_digits(input: &str, length: usize) -> Result {
        if input.len() == length && input.parse::<u64>().is_ok() {
            Ok(input)
        } else {
            Err("value not in allowed")
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Passport<'a> {
    byr: &'a str,
    ecl: &'a str,
    eyr: &'a str,
    hcl: &'a str,
    hgt: &'a str,
    iyr: &'a str,
    pid: &'a str,
    cid: Option<&'a str>,
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.

impl<'a> TryFrom<&'a str> for Passport<'a> {
    type Error = &'static str;

    fn try_from(value: &'a str) -> Result<Passport<'a>, Self::Error> {
        use validators::{in_range, is_color, number_digits, one_of, valid_height};
        let mut fields = HashMap::new();
        for entry in value.trim().split_whitespace() {
            let mid = entry.find(':');
            let mid = mid.ok_or("Missing : to delimit fields")?;
            let (key, val) = entry.split_at(mid);
            fields.insert(key, &val[1..]);
        }
        for field in &["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"] {
            fields.get(field).ok_or("missing field")?;
        }
        let passport = Passport {
            byr: in_range(fields["byr"], 1920..=2002)?,
            ecl: one_of(
                fields["ecl"],
                &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"],
            )?,
            eyr: in_range(fields["eyr"], 2020..=2030)?,
            hcl: is_color(fields["hcl"])?,
            hgt: valid_height(fields["hgt"])?,
            iyr: in_range(fields["iyr"], 2010..=2020)?,
            pid: number_digits(fields["pid"], 9)?,
            cid: fields.get("cid").copied(),
        };
        Ok(passport)
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let entries: Vec<_> = input.trim().split("\n\n").map(Passport::try_from).collect();
    let values_valid_count = entries.iter().filter(|p| p.is_ok()).count();
    println!("Values valid: {}", values_valid_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn passport<'a>(input: &'a str) -> Passport<'a> {
        Passport::try_from(input).unwrap()
    }

    #[test]
    fn test_passport_valid() {
        let result = passport(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
             byr:1937 iyr:2017 cid:147 hgt:183cm",
        );

        assert!(result.is_ok());

        let result = passport(
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
             hcl:#cfa07d byr:1929",
        );
        assert!(!result.is_err());

        let result = passport(
            "hcl:#ae17e1 iyr:2013\n\
             eyr:2024\n\
             ecl:brn pid:760753108 byr:1931\n\
             hgt:179cm",
        );
        assert!(result.is_ok());

        let result = passport(
            "hcl:#cfa07d eyr:2025 pid:166559648\n\
             iyr:2011 ecl:brn hgt:59in",
        );
        assert!(!result.is_err());
    }
}

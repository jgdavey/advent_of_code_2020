use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let station: Station = input.parse().unwrap();
    println!(
        "Error rate: {:?}",
        station.error_codes().iter().sum::<usize>()
    );

    let my_ticket = station.my_ticket();

    println!("Solved: {:#?}", my_ticket);

    let product: usize = my_ticket
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| v)
        .product();

    println!("Departure product: {:?}", product);
}

type Number = usize;
type Ticket = Vec<Number>;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Rule {
    name: String,
    ranges: Vec<RangeInclusive<Number>>,
}

impl Rule {
    fn is_valid(&self, number: Number) -> bool {
        self.ranges.iter().any(|r| r.contains(&number))
    }
}

#[derive(Debug)]
struct Station {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl FromStr for Station {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split("\n\n");
        let rule_section = sections.next().ok_or("Missing rules")?;
        let my_ticket_section = sections
            .next()
            .ok_or("Missing my ticket")?
            .lines()
            .nth(1)
            .ok_or("Missing my ticket")?;
        let nearby_ticket_section = sections
            .next()
            .ok_or("Missing nearby tickets")?
            .lines()
            .skip(1);
        let parse_number = |s: &str| s.parse::<Number>().map_err(|_| "Not a number");

        let mut rules = vec![];
        for line in rule_section.lines() {
            let mut kv = line.splitn(2, ": ");
            let name = kv.next().ok_or("Missing rule name")?.to_string();
            let v = kv.next().ok_or("Missing rule values")?;
            let mut ranges = vec![];
            for r in v.split(" or ") {
                let mut parts = r.splitn(2, '-');
                let mut take = || parts.next().ok_or("Invalid range").and_then(parse_number);
                let a = take()?;
                let b = take()?;
                ranges.push(RangeInclusive::new(a, b))
            }
            rules.push(Rule { name, ranges });
        }

        let parse_ticket = |s: &str| {
            s.split(',')
                .map(parse_number)
                .collect::<Result<Vec<_>, _>>()
        };
        let my_ticket = parse_ticket(my_ticket_section)?;
        let nearby_tickets = nearby_ticket_section
            .map(parse_ticket)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Station {
            rules,
            my_ticket,
            nearby_tickets,
        })
    }
}

impl Station {
    fn errors_for_ticket(&self, ticket: &Ticket) -> Vec<Number> {
        let mut errors = vec![];
        for &number in ticket {
            if !self.rules.iter().any(|rule| rule.is_valid(number)) {
                errors.push(number);
            }
        }
        errors
    }

    fn error_codes(&self) -> Vec<Number> {
        self.nearby_tickets
            .iter()
            .map(|t| self.errors_for_ticket(t))
            .flatten()
            .collect()
    }

    /// Returns list of (field, Set<rule idx>) tuples
    fn possibilities(&self) -> Vec<(usize, HashSet<usize>)> {
        let valid_tickets: Vec<_> = self
            .nearby_tickets
            .iter()
            .filter(|ticket| self.errors_for_ticket(ticket).is_empty())
            .collect();
        let mut possibilities = vec![];
        for field in 0..valid_tickets[0].len() {
            let rules = self
                .rules
                .iter()
                .enumerate()
                .filter(|(_, rule)| valid_tickets.iter().all(|t| rule.is_valid(t[field])))
                .map(|(idx, _)| idx)
                .collect();
            possibilities.push((field, rules));
        }
        possibilities
    }

    /// Returns Rules in field order
    fn solve(&self) -> Vec<&Rule> {
        let mut possibilities = self.possibilities();
        possibilities.sort_by_key(|(_, r)| r.len());

        // assume a single solution
        let mut assigned = HashSet::new();
        let mut solved = vec![];
        for (field, rules) in possibilities {
            let diff: HashSet<_> = rules.difference(&assigned).cloned().collect();
            if diff.len() != 1 {
                panic!("I dunno");
            }
            let rule = diff.iter().next().unwrap();
            assigned.insert(*rule);
            solved.push((field, *rule));
        }
        solved.sort_by_key(|&(field, _)| field);
        solved.iter().map(|&(_, rule)| &self.rules[rule]).collect()
    }

    fn my_ticket(&self) -> HashMap<String, usize> {
        self.solve()
            .iter()
            .enumerate()
            .map(|(field, rule)| (rule.name.clone(), self.my_ticket[field]))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! set {
        ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
            {
                let mut temp_set = HashSet::new();  // Create a mutable HashSet
                $(
                    temp_set.insert($x); // Insert each item matched into the HashSet
                )*
                    temp_set // Return the populated HashSet
            }
        };
    }

    #[test]
    fn test_part_1() {
        let input = "class: 1-3 or 5-7\n\
                     row: 6-11 or 33-44\n\
                     seat: 13-40 or 45-50\n\
                     \n\
                     your ticket:\n\
                     7,1,14\n\
                     \n\
                     nearby tickets:\n\
                     7,3,47\n\
                     40,4,50\n\
                     55,2,20\n\
                     38,6,12";

        let station: Station = input.parse().unwrap();
        assert_eq!(station.error_codes(), vec![4, 55, 12]);
    }

    #[test]
    fn test_part_2() {
        let input = "class: 0-1 or 4-19\n\
                     row: 0-5 or 8-19\n\
                     seat: 0-13 or 16-19\n\
                     \n\
                     your ticket:\n\
                     11,12,13\n\
                     \n\
                     nearby tickets:\n\
                     3,9,18\n\
                     15,1,5\n\
                     5,14,9";
        let station: Station = input.parse().unwrap();
        let class = Rule {
            name: "class".to_string(),
            ranges: vec![0..=1, 4..=19],
        };
        let row = Rule {
            name: "row".to_string(),
            ranges: vec![0..=5, 8..=19],
        };
        let seat = Rule {
            name: "seat".to_string(),
            ranges: vec![0..=13, 16..=19],
        };
        let expected = vec![(0, set! {1}), (1, set! {0, 1}), (2, set! {0, 1, 2})];
        assert_eq!(station.possibilities(), expected);
        assert_eq!(station.solve(), vec![&row, &class, &seat]);
    }
}

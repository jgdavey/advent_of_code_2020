use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let station: Station = input.parse().unwrap();
    println!(
        "Error rate: {:?}",
        station.error_codes().iter().sum::<usize>()
    );
}

type Number = usize;
type Ticket = Vec<Number>;

#[derive(Debug)]
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
        println!("{:?}", station);
        assert_eq!(station.error_codes(), vec![4, 55, 12]);
    }
}

use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Content {
    color: String,
    count: usize,
}

#[derive(Debug)]
struct Rule {
    color: String,
    contents: Vec<Content>,
}

impl Rule {
    pub fn color_count(&self, color: &str) -> Option<usize> {
        if let Some(content) = self.contents.iter().find(|c| c.color == color) {
            return Some(content.count);
        }
        None
    }
}

type RuleSet = HashMap<String, Rule>;

fn traverse_to(ruleset: &RuleSet, start: &str, needle: &str) -> Option<usize> {
    let rule = ruleset.get(start)?;
    if let Some(v) = rule.color_count(needle) {
        return Some(v);
    }
    let counts = rule
        .contents
        .iter()
        .filter_map(|content| traverse_to(ruleset, &content.color, needle))
        .collect::<Vec<_>>();
    if counts.is_empty() {
        None
    } else {
        Some(counts.iter().sum())
    }
}

fn count_below(ruleset: &RuleSet, start: &Rule, multiplier: usize) -> usize {
    let mut sum = 0;
    for content in start.contents.iter() {
        let rule = &ruleset[&content.color];
        sum +=
            (multiplier * content.count) + count_below(ruleset, rule, multiplier * content.count);
    }
    sum
}

fn parse_rules(input: &str) -> HashMap<String, Rule> {
    lazy_static! {
        static ref CONTENTS: Regex = Regex::new("(\\d+) (\\w+ \\w+) bags?,? ?").unwrap();
        static ref LINE: Regex = Regex::new("(?m)(\\w+ \\w+) bags contain (.*).$").unwrap();
    }

    let mut rules = HashMap::new();

    for cap in LINE.captures_iter(input) {
        rules.insert(
            cap[1].to_string(),
            Rule {
                color: cap[1].to_string(),
                contents: CONTENTS
                    .captures_iter(&cap[2])
                    .map(|c| Content {
                        color: c[2].to_string(),
                        count: c[1].parse().unwrap(),
                    })
                    .collect(),
            },
        );
    }
    rules
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let rules = parse_rules(&input);

    // for each key, count those that have some path to "shiny gold" bags
    let mut count = 0;
    for rule in rules.keys() {
        if let Some(_n) = traverse_to(&rules, rule, "shiny gold") {
            count += 1;
        }
    }

    let bag_total = count_below(&rules, &rules["shiny gold"], 1);

    // for the "shiny gold" bag, count all bags necessary
    println!(
        "All Rules: {},\tTraversible: {},\tBag total {}",
        rules.len(),
        count,
        bag_total
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule() {
        let rules = parse_rules(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                                 bright white bags contain 1 shiny gold bag.\n\
                                 dotted black bags contain no other bags.",
        );
        let rule0 = &rules["light red"];
        let rule1 = &rules["bright white"];
        let rule2 = &rules["dotted black"];

        assert_eq!(rule0.contents[0].color, "bright white");
        assert_eq!(rule0.contents[0].count, 1);
        assert_eq!(rule0.contents[1].color, "muted yellow");
        assert_eq!(rule0.contents[1].count, 2);

        assert_eq!(rule1.contents[0].color, "shiny gold");
        assert_eq!(rule1.contents[0].count, 1);

        assert_eq!(rule2.contents.len(), 0);
    }

    #[test]
    fn test_below() {
        let rules = parse_rules(
            "shiny gold bags contain 2 dark red bags.\n\
                                 dark red bags contain 2 dark orange bags.\n\
                                 dark orange bags contain 2 dark yellow bags.\n\
                                 dark yellow bags contain 2 dark green bags.\n\
                                 dark green bags contain 2 dark blue bags.\n\
                                 dark blue bags contain 2 dark violet bags.\n\
                                 dark violet bags contain no other bags.",
        );
        let count = count_below(&rules, &rules["shiny gold"], 1);
        assert_eq!(count, 126);
    }
}

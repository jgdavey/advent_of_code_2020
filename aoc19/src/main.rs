use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut splitter = input.splitn(2, "\n\n");
    let mut rules: RuleSet = splitter.next().unwrap().parse().unwrap();
    let lines = splitter.next().unwrap();
    println!("All lines: {}", lines.lines().count());
    println!(
        "Count: {}",
        lines.lines().filter(|l| rules.valid(l)).count()
    );

    // Part 2 rules
    // 8: 42 8 | 42
    // 11: 42 31 | 42 11 31
    rules.rules.insert(8, "42 | 42 8".parse().unwrap());
    rules.rules.insert(11, "42 31 | 42 11 31 ".parse().unwrap());

    println!(
        "Count: {}",
        lines.lines().filter(|l| rules.valid(l)).count()
    );
}

type RuleId = u32;

#[derive(Debug)]
enum Rule {
    Literal(char),
    Sequence(Vec<RuleId>),
    Alt(Vec<Vec<RuleId>>),
}

struct RuleSet {
    rules: HashMap<RuleId, Rule>,
}

impl RuleSet {
    fn consume<'a>(&self, s: &[&'a str], rule: &Rule) -> Result<Vec<&'a str>, ()> {
        if s.is_empty() {
            return Err(());
        }
        match rule {
            Rule::Literal(c) => {
                let p = s
                    .iter()
                    .filter_map(|s| {
                        s.chars()
                            .next()
                            .and_then(|ch| if *c == ch { Some(&s[1..]) } else { None })
                    })
                    .collect::<Vec<_>>();
                if p.is_empty() {
                    Err(())
                } else {
                    Ok(p)
                }
            }
            Rule::Sequence(rule_ids) => {
                let p = s
                    .iter()
                    .filter_map(|s| {
                        rule_ids
                            .iter()
                            .map(|id| self.rules.get(id).unwrap())
                            .try_fold(vec![*s], |st, el| self.consume(st.as_slice(), el).ok())
                    })
                    .flatten()
                    .collect::<Vec<_>>();
                if p.is_empty() {
                    Err(())
                } else {
                    Ok(p)
                }
            }
            Rule::Alt(rule_ids) => {
                let mut possibles = vec![];
                for rule in rule_ids.iter().map(|ids| Rule::Sequence(ids.clone())) {
                    match self.consume(s, &rule) {
                        Ok(p) => possibles.extend(p),
                        _ => (),
                    }
                }
                if possibles.is_empty() {
                    Err(())
                } else {
                    Ok(possibles)
                }
            }
        }
    }

    fn valid(&self, s: &str) -> bool {
        if let Some(primary) = self.rules.get(&0) {
            match self.consume(&vec![s], primary) {
                Ok(r) => r.iter().any(|s| s == &""),
                _ => false,
            }
        } else {
            false
        }
    }
}

fn to_rule_ids(s: &str) -> Vec<RuleId> {
    s.trim()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

impl FromStr for Rule {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rule = if let Some('"') = s.chars().next() {
            Rule::Literal(s.chars().nth(1).ok_or("Missing char after \"")?)
        } else {
            let mut rules: Vec<_> = s.split('|').map(|part| to_rule_ids(part.trim())).collect();
            if rules.len() == 1 {
                Rule::Sequence(rules.remove(0))
            } else {
                Rule::Alt(rules)
            }
        };
        Ok(rule)
    }
}

impl FromStr for RuleSet {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = HashMap::with_capacity(s.lines().count());
        for line in s.lines() {
            let idx = line.find(':').ok_or("Missing colon")?;
            let (id, rest) = line.split_at(idx);
            let rest = &rest[2..];
            let id = id.parse().map_err(|_| "Bad id")?;
            rules.insert(id, rest.parse()?);
        }
        Ok(RuleSet { rules })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_rule() {
        let input = "0: 4 1 5\n\
                     1: 2 3 | 3 2\n\
                     2: 4 4 | 5 5\n\
                     3: 4 5 | 5 4\n\
                     4: \"a\"\n\
                     5: \"b\"";
        let ruleset: RuleSet = input.parse().unwrap();
        assert!(ruleset.valid("ababbb"));
        assert!(ruleset.valid("abbbab"));
        assert!(!ruleset.valid("bababa"));
        assert!(!ruleset.valid("aaabbb"));
        assert!(!ruleset.valid("aaaabbb"));
    }

    #[test]
    fn test_part_2() {
        let input = "42: 9 14 | 10 1\n\
                     9: 14 27 | 1 26\n\
                     10: 23 14 | 28 1\n\
                     1: \"a\"\n\
                     11: 42 31\n\
                     5: 1 14 | 15 1\n\
                     19: 14 1 | 14 14\n\
                     12: 24 14 | 19 1\n\
                     16: 15 1 | 14 14\n\
                     31: 14 17 | 1 13\n\
                     6: 14 14 | 1 14\n\
                     2: 1 24 | 14 4\n\
                     0: 8 11\n\
                     13: 14 3 | 1 12\n\
                     15: 1 | 14\n\
                     17: 14 2 | 1 7\n\
                     23: 25 1 | 22 14\n\
                     28: 16 1\n\
                     4: 1 1\n\
                     20: 14 14 | 1 15\n\
                     3: 5 14 | 16 1\n\
                     27: 1 6 | 14 18\n\
                     14: \"b\"\n\
                     21: 14 1 | 1 14\n\
                     25: 1 1 | 1 14\n\
                     22: 14 14\n\
                     8: 42\n\
                     26: 14 22 | 1 20\n\
                     18: 15 15\n\
                     7: 14 5 | 1 21\n\
                     24: 14 1";

        let testers = "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\n\
                       bbabbbbaabaabba\n\
                       babbbbaabbbbbabbbbbbaabaaabaaa\n\
                       aaabbbbbbaaaabaababaabababbabaaabbababababaaa\n\
                       bbbbbbbaaaabbbbaaabbabaaa\n\
                       bbbababbbbaaaaaaaabbababaaababaabab\n\
                       ababaaaaaabaaab\n\
                       ababaaaaabbbaba\n\
                       baabbaaaabbaaaababbaababb\n\
                       abbbbabbbbaaaababbbbbbaaaababb\n\
                       aaaaabbaabaaaaababaa\n\
                       aaaabbaaaabbaaa\n\
                       aaaabbaabbaaaaaaabbbabbbaaabbaabaaa\n\
                       babaaabbbaaabaababbaabababaaab\n\
                       aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

        let mut ruleset: RuleSet = input.parse().unwrap();
        let valid1: Vec<_> = testers.lines().filter(|line| ruleset.valid(line)).collect();
        assert_eq!(
            valid1,
            vec!["bbabbbbaabaabba", "ababaaaaaabaaab", "ababaaaaabbbaba"]
        );

        ruleset.rules.insert(8, "42 | 42 8".parse().unwrap());
        //ruleset.rules.insert(11, "42 31 | 42 11 31".parse().unwrap());

        ruleset.rules.insert(11, "42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31 | 42 42 42 42 42 31 31 31 31 31 | 42 42 42 42 42 42 31 31 31 31 31 31 | 42 42 42 42 42 42 42 31 31 31 31 31 31 31 | 42 42 42 42 42 42 42 42 31 31 31 31 31 31 31 31 | 42 42 42 42 42 42 42 42 42 31 31 31 31 31 31 31 31 31 | 42 42 42 42 42 42 42 42 42 42 31 31 31 31 31 31 31 31 31 31".parse().unwrap());

        let valid2: Vec<_> = testers.lines().filter(|line| ruleset.valid(line)).collect();
        assert_eq!(
            valid2,
            vec![
                "bbabbbbaabaabba",
                "babbbbaabbbbbabbbbbbaabaaabaaa",
                "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
                "bbbbbbbaaaabbbbaaabbabaaa",
                "bbbababbbbaaaaaaaabbababaaababaabab",
                "ababaaaaaabaaab",
                "ababaaaaabbbaba",
                "baabbaaaabbaaaababbaababb",
                "abbbbabbbbaaaababbbbbbaaaababb",
                "aaaaabbaabaaaaababaa",
                "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
                "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            ]
        );
    }
}

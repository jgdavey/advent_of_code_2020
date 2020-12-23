use std::str::FromStr;
use std::collections::{HashSet, HashMap, BTreeMap, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let menu: Menu = input.parse().unwrap();
    println!("Non-allergen ingredients listed: {}", menu.non_allergen_ingredients());
    let solved = menu.solve();
    println!("Allergen ingredients: {:?}", solved);
    let list = solved.values().cloned().collect::<Vec<_>>().as_slice().join(",");
    println!("List: {:?}", list);
}

#[derive(Debug)]
struct Entry {
    ingredients: HashSet<String>,
    allergens: HashSet<String>
}

impl FromStr for Entry {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.splitn(2, " (contains ");
        let ing_list = parts.next().ok_or("Missing ingredient list")?;
        let alg_list = parts.next().ok_or("Missing allergen list")?.split(")").next().unwrap();
        let ingredients = ing_list.trim().split(" ").map(|s| s.to_string()).collect::<HashSet<String>>();
        let allergens = alg_list.split(", ").map(|s| s.to_string()).collect::<HashSet<String>>();
        Ok(Entry { ingredients, allergens })
    }
}

#[derive(Debug)]
struct Menu {
    entries: Vec<Entry>,
}

impl Menu {
    fn possibles(&self) -> HashMap<String, HashSet<String>> {
        let mut possibles: HashMap<String, HashSet<String>> = HashMap::new();
        for entry in &self.entries {
            for allergen in &entry.allergens {
                possibles
                    .entry(allergen.to_string())
                    .and_modify(|ing| { *ing = ing.intersection(&entry.ingredients).cloned().collect(); })
                    .or_insert_with(|| entry.ingredients.clone());
            }
        }
        possibles
    }

    fn allergen_ingredients(&self) -> HashSet<String> {
        let mut list = HashSet::new();
        for ing in self.possibles().values() {
            list.extend(ing.iter().cloned());
        }
        list
    }

    fn non_allergen_ingredients(&self) -> usize {
        let allergens = self.allergen_ingredients();
        self.entries.iter().map(|e| e.ingredients.difference(&allergens).count()).sum()
    }

    fn solve(&self) -> BTreeMap<String, String> {
        let mut poss: Vec<_> = self.possibles().into_iter().collect();
        poss.sort_by_key(|(_, v)| v.len());
        let mut work: VecDeque<_> = poss.into();
        let mut out = BTreeMap::new();
        let mut found = HashSet::new();
        while !work.is_empty() {
            let (allergen, v) = work.pop_front().unwrap();
            if v.difference(&found).count() != 1 {
                work.push_back((allergen, v));
            } else {
                let ingredient = v.difference(&found).next().unwrap().to_string();
                out.insert(allergen.clone(), ingredient.clone());
                found.insert(ingredient);
            }
        }
        out
    }
}

impl FromStr for Menu {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = vec![];
        for line in s.lines() {
            let entry: Entry = line.parse()?;
            entries.push(entry);
        }
        Ok(Menu { entries })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_allergens() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n\
                     trh fvjkl sbzzf mxmxvkd (contains dairy)\n\
                     sqjhc fvjkl (contains soy)\n\
                     sqjhc mxmxvkd sbzzf (contains fish)";

        let menu: Menu = input.parse().unwrap();
        assert_eq!(menu.non_allergen_ingredients(), 5);
    }

    #[test]
    fn test_solve() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n\
                     trh fvjkl sbzzf mxmxvkd (contains dairy)\n\
                     sqjhc fvjkl (contains soy)\n\
                     sqjhc mxmxvkd sbzzf (contains fish)";

        let menu: Menu = input.parse().unwrap();
        let mut expected = BTreeMap::new();
        expected.insert("dairy".to_string(), "mxmxvkd".to_string());
        expected.insert("fish".to_string(), "sqjhc".to_string());
        expected.insert("soy".to_string(), "fvjkl".to_string());
        assert_eq!(menu.solve(), expected);
    }
}

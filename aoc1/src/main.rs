use std::fs::read_to_string;
use std::env;

fn summing_to(numbers: &[usize], total: usize) -> Option<Vec<usize>> {
    for (i, &n1) in numbers.iter().enumerate() {
        let t1 = total - n1;
        for (j, &n2) in numbers.iter().skip(i + 1).enumerate() {
            if n2 > t1 {
                continue;
            }
            let t2 = t1 - n2;
            if let Some(&n3) = numbers.iter().skip(i + j + 2).find(|n| **n == t2) {
                return Some(vec![n1, n2, n3]);
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let default = "input.txt".to_string();
    let input = args.get(1).unwrap_or(&default);
    println!("Input: {}", input);

    let string = read_to_string(input)?;
    let mut nums = vec![];
    for line in string.lines() {
        nums.push(line.parse::<usize>()?);
    }
    let result = summing_to(nums.as_slice(), 2020).ok_or_else(|| "None")?;
    let product = result.iter().fold(1, |a, b| a * b);
    println!("{:?} *= {}", result, product);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_small() {
        let list = vec![
            1721,
            979,
            366,
            299,
            675,
            1456
        ];
        let result = summing_to(&list, 2020);
        assert_eq!(result, Some(vec![1721, 299]));
    }
}

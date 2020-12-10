use itertools::Itertools;

// really this is off by one from the "canonical" trib sequence, but
// more convenient for this problem.
fn tribonacci(n: usize) -> usize {
    if n < 2 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    let mut c = 1;

    let mut curr = 0;
    for _ in 1..n {
        curr = a + b + c;
        a = b;
        b = c;
        c = curr;
    }
    curr
}

fn sort_adapters(adapters: &mut Vec<u16>) {
    // Add outlet
    adapters.push(0);
    adapters.sort_unstable();
    // Add device
    adapters.push(adapters.last().unwrap() + 3);
}

fn jolt_diff_summary(adapters: &[u16]) -> (u16, u16, u16) {
    let mut ones = 0u16;
    let mut twos = 0u16;
    let mut threes = 0u16;
    for diff in adapters.windows(2).map(|s| s[1] - s[0]) {
        match diff {
            1 => ones += 1,
            2 => twos += 1,
            3 => threes += 1,
            _ => (),
        }
    }
    (ones, twos, threes)
}

fn arrangements(adapters: &[u16]) -> usize {
    let mut possibilities = 1;
    for (ones, group) in &adapters
        .windows(2)
        .map(|s| s[1] - s[0])
        .group_by(|&n| n == 1)
    {
        if ones {
            possibilities *= tribonacci(group.count());
        }
    }
    possibilities
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("error reading input.txt");
    let mut result: Result<Vec<_>, _> = input.lines().map(|line| line.parse::<u16>()).collect();
    match result {
        Ok(ref mut adapters) => {
            sort_adapters(adapters);
            let (ones, twos, threes) = jolt_diff_summary(&adapters);
            println!("Twos: {}", twos);
            println!("Summary: {} * {} = {}", ones, threes, ones * threes);
            println!("Possible arrangements: {}", arrangements(&adapters));
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joltage_differences() {
        let mut adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        sort_adapters(&mut adapters);
        assert_eq!(jolt_diff_summary(&adapters), (22, 0, 10));
    }

    #[test]
    fn test_arrangements() {
        let mut adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        sort_adapters(&mut adapters);
        assert_eq!(arrangements(&adapters), 19208);
    }

    #[test]
    fn test_arrangements_small1() {
        let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        sort_adapters(&mut adapters);
        assert_eq!(arrangements(&adapters), 8);
    }

    #[test]
    fn test_arrangements_small2() {
        let mut adapters = vec![1, 2, 3, 4];
        sort_adapters(&mut adapters);

        // sequence: 0, 1, 1, 2, 4, 7

        // arrangement    diffs
        // 0              -

        // 0 1            1

        // 0 1 2          1 1
        // 0 2            -2-

        // 0 1 2 3        1 1 1
        // 0 1 3          1 -2-
        // 0 2 3          -2- 1
        // 0 3              3

        // 0 1 2 3 4      1 1 1 1
        // 0 1 2 4        1 1 -2-
        // 0 1 3 4        1 -2- 1
        // 0 1 4          1 - 3 -
        // 0 2 3 4        -2- 1 1
        // 0 2 4          -2- -2-
        // 0 3 4          - 3 - 1

        assert_eq!(arrangements(&adapters), 7);
    }

    #[test]
    fn test_tribonacci() {
        assert_eq!(tribonacci(0), 1);
        assert_eq!(tribonacci(1), 1);
        assert_eq!(tribonacci(2), 2);
        assert_eq!(tribonacci(3), 4);
        assert_eq!(tribonacci(4), 7);
        assert_eq!(tribonacci(5), 13);
    }
}

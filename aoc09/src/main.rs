use std::cmp::Ordering;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let numbers: Vec<_> = input.lines().map(|s| s.parse::<isize>().unwrap()).collect();
    let (idx, target) = numbers.iter().enumerate().skip(25).find(|&(i, target)| {
        let previous = &numbers[(i - 25)..i];
        let mut okay = false;
        for (j, num) in previous.iter().enumerate() {
            let difference = target - num;
            if let Some(_) = previous[j+1..].iter().find(|&n| *n == difference) {
                okay = true;
                break;
            }
        }
        !okay
    }).unwrap();

    println!("Target: {:?}", target);
    for i in 0..idx {
        let mut sum = 0;
        for (j, num) in numbers[i..idx].iter().enumerate() {
            sum += num;
            match target.cmp(&sum) {
                Ordering::Equal => {
                    let slice = &numbers[i..=(j+i)];
                    let min = slice.iter().min().unwrap();
                    let max = slice.iter().max().unwrap();
                    println!("Found: {} + {} = {}", min, max, min + max);
                    return;
                },
                Ordering::Less => break,
                Ordering::Greater => continue
            }
        }
    }
}

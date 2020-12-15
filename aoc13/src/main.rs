fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let timestamp = input.lines().next().unwrap().parse::<usize>().unwrap();
    let mut bus_ids: Vec<_> = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    bus_ids.sort_unstable();

    let smallest = bus_ids[0];

    'outer: for t in timestamp..(timestamp + smallest) {
        for id in &bus_ids {
            if t % id == 0 {
                println!("{} * {} = {}", id, t - timestamp, id * (t - timestamp));
                break 'outer;
            }
        }
    }
}

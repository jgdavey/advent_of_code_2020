fn bus_id_list(s: &str) -> Vec<(usize, usize)> {
    s.split(',')
        .enumerate()
        .filter_map(|(i, s)| match s.parse::<usize>() {
            Ok(id) => Some((i, id)),
            Err(_) => None,
        })
        .collect()
}

fn earliest(ids: &[(usize, usize)]) -> Option<usize> {
    if ids.is_empty() {
        return None;
    }

    let mut time = 1;
    let mut interval = 1;
    let mut idx = 0;

    while idx < ids.len() {
        time += interval;

        let (offset, id) = ids[idx];

        if (time + offset) % id == 0 {
            // Only works if IDs are coprime
            interval *= id;
            idx += 1;
        }
    }

    Some(time)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let bus_ids = bus_id_list(input.lines().skip(1).next().unwrap());

    println!("Earliest {:?}", earliest(&bus_ids));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        assert_eq!(earliest(&bus_id_list("17,x,13,19")), Some(3417));
        assert_eq!(earliest(&bus_id_list("67,7,59,61")), Some(754018));
        assert_eq!(earliest(&bus_id_list("67,x,7,59,61")), Some(779210));
        assert_eq!(earliest(&bus_id_list("67,7,x,59,61")), Some(1261476));
        assert_eq!(earliest(&bus_id_list("1789,37,47,1889")), Some(1202161486));
    }
}

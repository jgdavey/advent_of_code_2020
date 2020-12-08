pub fn decode(seat: &str) -> usize {
    let binary: String = seat
        .chars()
        .map(|c| match c {
            'B' | 'R' => '1',
            'F' | 'L' => '0',
            _ => c,
        })
        .collect();
    usize::from_str_radix(&binary, 2).unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut seat_ids: Vec<_> = input.lines().map(decode).collect();
    seat_ids.sort();
    for (i, seat) in seat_ids.iter().skip(1).enumerate() {
        if seat - seat_ids[i] > 1 {
            print!("Seat: {}", seat - 1);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::decode;

    #[test]
    fn test_decode_seat_id() {
        let seat = "FBFBBFFRLR";
        let decoded = decode(seat);
        assert_eq!(decoded, 357);
    }
}

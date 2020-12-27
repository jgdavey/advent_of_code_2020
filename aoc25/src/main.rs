fn main() {
    let pk1 = 12320657;
    let pk2 = 9659666;
    let l1 = find_loop_size(7, pk1).unwrap();
    let l2 = find_loop_size(7, pk2).unwrap();
    let e1 = transform_loop(pk1, l2);
    let e2 = transform_loop(pk2, l1);

    println!("Encryption key(s): {}, {}", e1, e2);
}

// The handshake used by the card and the door involves an operation
// that transforms a subject number. To transform a subject number,
// start with the value 1. Then, a number of times called the loop
// size, perform the following steps:

// 1. Set the value to itself multiplied by the subject number.
// 2. Set the value to the remainder after dividing the value by 20201227.

// The card always uses a specific, secret loop size when it
// transforms a subject number. The door always uses a different,
// secret loop size.

// The cryptographic handshake works like this:

// - The card transforms the subject number of 7 according to the card's
// secret loop size. The result is called the card's public key.
// - The door transforms the subject number of 7 according to the door's
// secret loop size. The result is called the door's public key. The
// card and door use the wireless RFID signal to transmit the two
// public keys (your puzzle input) to the other device. Now, the card
// has the door's public key, and the door has the card's public key.
// Because you can eavesdrop on the signal, you have both public keys,
// but neither device's loop size.
// - The card transforms the subject number of the door's public key
// according to the card's loop size. The result is the encryption key.
// - The door transforms the subject number of the card's public key
// according to the door's loop size. The result is the same
// encryption key as the card calculated.

// If you can use the two public keys to determine each device's loop
// size, you will have enough information to calculate the secret
// encryption key that the card and door use to communicate; this
// would let you send the unlock command directly to the door!

fn transform(mut value: usize, subject: usize) -> usize {
    value *= subject;
    value = value % 20201227;
    value
}

fn transform_loop(subject: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = transform(value, subject);
    }
    value
}

fn find_loop_size(subject: usize, public_key: usize) -> Result<usize, String> {
    let mut value = 1;
    for l in 1..100_000_000 {
        value = transform(value, subject);
        if value == public_key {
            return Ok(l);
        }
    }
    Err(String::from(
        "Unable to find loop size after many iterations",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    // For example, suppose you know that the card's public key is
    // 5764801. With a little trial and error, you can work out that
    // the card's loop size must be 8, because transforming the initial
    // subject number of 7 with a loop size of 8 produces 5764801.

    // Then, suppose you know that the door's public key is 17807724. By
    // the same process, you can determine that the door's loop size is
    // 11, because transforming the initial subject number of 7 with a
    // loop size of 11 produces 17807724.

    #[test]
    fn test_transform() {
        assert_eq!(find_loop_size(7, 17807724), Ok(11));
        assert_eq!(find_loop_size(7, 5764801), Ok(8));

        assert_eq!(transform_loop(17807724, 8), 14897079);
        assert_eq!(transform_loop(5764801, 11), 14897079);
    }
}

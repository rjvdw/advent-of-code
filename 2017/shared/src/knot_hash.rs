use std::collections::VecDeque;

pub fn compute_sparse_hash(lengths: &[u8], rope_length: usize, rounds: usize) -> VecDeque<u8> {
    let mut rope = VecDeque::with_capacity(rope_length);
    for i in 0..rope_length {
        rope.push_back(i as u8);
    }

    let mut current_position: usize = 0;
    let mut skip_size: usize = 0;

    for _ in 0..rounds {
        for &length in lengths {
            let length = length as usize;
            for i in 0..length / 2 {
                rope.swap(i, length - i - 1);
            }
            let offset = (length + skip_size) % rope_length;
            current_position = (current_position + offset) % rope_length;
            skip_size += 1;
            rope.rotate_left(offset);
        }
    }
    rope.rotate_right(current_position);
    rope
}

pub fn compute_knot_hash(lengths: &[u8]) -> u128 {
    let sparse_hash = compute_sparse_hash(lengths, 256, 64);
    let mut knot_hash = 0;
    let mut ch = 0;
    for (idx, b) in sparse_hash.iter().enumerate() {
        if idx != 0 && idx % 16 == 0 {
            knot_hash <<= 8;
            knot_hash += ch as u128;
            ch = 0;
        }
        ch ^= *b;
    }
    knot_hash <<= 8;
    knot_hash += ch as u128;
    knot_hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_sparse_hash() {
        let mut expected: VecDeque<u8> = VecDeque::new();
        expected.push_back(3);
        expected.push_back(4);
        expected.push_back(2);
        expected.push_back(1);
        expected.push_back(0);
        assert_eq!(compute_sparse_hash(&[3, 4, 1, 5], 5, 1), expected);
    }

    #[test]
    fn test_compute_knot_hash() {
        assert_eq!(compute_knot_hash(&[]), 0);
        assert_eq!(
            compute_knot_hash(&[3, 4, 1, 5]),
            0xe31f007e7b3a3c0707071b1e070610ec
        );
    }
}

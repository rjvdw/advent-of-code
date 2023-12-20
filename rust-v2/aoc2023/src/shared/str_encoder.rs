//! Encode strings to values that have a Size, so they are easier to work with.

/// Encode a string of at most four ascii characters to its binary representation.
pub fn encode_str(s: &str) -> u32 {
    let mut encoded = 0;
    for b in s.bytes() {
        encoded <<= 8;
        encoded += b as u32;
    }
    encoded
}

/// Decode the binary representation of a string of at most four ascii characters to a string.
pub fn decode_str(s: u32) -> String {
    let mut encoded = s;
    let mut res = String::new();
    while encoded > 0 {
        let ch = ((encoded & 0b11111111) as u8) as char;
        res.push(ch);
        encoded >>= 8;
    }
    res.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode_str("a"), 0b00000000_00000000_00000000_01100001);
        assert_eq!(encode_str("ab"), 0b00000000_00000000_01100001_01100010);
        assert_eq!(encode_str("abc"), 0b00000000_01100001_01100010_01100011);
        assert_eq!(encode_str("abcd"), 0b01100001_01100010_01100011_01100100);
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode_str(0b00000000_00000000_00000000_01100001), "a");
        assert_eq!(decode_str(0b00000000_00000000_01100001_01100010), "ab");
        assert_eq!(decode_str(0b00000000_01100001_01100010_01100011), "abc");
        assert_eq!(decode_str(0b01100001_01100010_01100011_01100100), "abcd");
    }
}

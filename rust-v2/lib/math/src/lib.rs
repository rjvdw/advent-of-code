//! Useful mathematical operations.

/// Multiplies two numbers with a given modulus.
#[allow(clippy::many_single_char_names)]
pub fn mul_mod(mut a: u64, mut b: u64, modulus: u64) -> u64 {
    // https://en.wikipedia.org/wiki/Modular_arithmetic#Example_implementations

    let mut result = 0_u64;
    let mp2 = modulus >> 1;
    a %= modulus;
    b %= modulus;

    for _ in 0..64 {
        result = if result > mp2 {
            (result << 1) - modulus
        } else {
            result << 1
        };
        if a & 0x8000_0000_0000_0000_u64 != 0 {
            result += b;
        }
        if result > modulus {
            result -= modulus;
        }
        a <<= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    mod mul_mod {
        use super::*;

        #[test]
        fn test_mul_mod() {
            assert_eq!(mul_mod(10, 10, 7), 2);
            assert_eq!(mul_mod(u64::MAX, 2, 7), 2);
        }
    }
}

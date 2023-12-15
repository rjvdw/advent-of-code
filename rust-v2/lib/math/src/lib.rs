//! Useful mathematical operations.

use ops::{BitAnd, Shl, ShlAssign, Shr, Sub};
use std::ops;
use std::ops::{AddAssign, Div, Mul, Rem, RemAssign, SubAssign};

/// Computes the greatest common divisor for numbers a and b.
/// TODO: May return negative values.
#[allow(clippy::many_single_char_names)]
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + Eq + Rem<T, Output = T> + Default,
{
    let zero = T::default();
    let mut ab = (a, b);
    while ab.1 != zero {
        ab = (ab.1, ab.0 % ab.1);
    }
    ab.0
}

/// Computes the least common multiple of a and b.
#[allow(clippy::many_single_char_names)]
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy + Eq + Mul<T, Output = T> + Div<T, Output = T> + Rem<T, Output = T> + Default,
{
    let g = gcd(a, b);
    (a / g) * b
}

/// Solve the chinese remainder theorem for (n1, a1) and (n2, a2). We assume that:
/// * n1 and n2 are coprime
/// * n1 and n2 are no more than 63 bits (as they are converted to i64)
#[allow(clippy::many_single_char_names)]
pub fn solve_crt((n_1, a_1): (u64, u64), (n_2, a_2): (u64, u64)) -> u64 {
    let prod = n_1 * n_2;
    let (m_1, m_2) = bezout_coefficients(n_1 as i64, n_2 as i64);

    // since n1 and n2 are coprime, either m1 or m2 is negative (but not both)

    let (p_1, p_2) = (
        crt_mod_mult_helper(a_1, m_2, n_2, prod),
        crt_mod_mult_helper(a_2, m_1, n_1, prod),
    );

    if m_1 < 0 {
        (p_1 + prod - p_2) % prod
    } else {
        (p_2 + prod - p_1) % prod
    }
}

#[allow(clippy::many_single_char_names)]
fn crt_mod_mult_helper(a: u64, m: i64, n: u64, modulus: u64) -> u64 {
    mul_mod(mul_mod(a, m.unsigned_abs(), modulus), n, modulus)
}

pub trait MulModCompatible:
    Copy
    + Default
    + From<u8>
    + Eq
    + Ord
    + RemAssign<Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + BitAnd<Self, Output = Self>
    + Shl<Self, Output = Self>
    + ShlAssign<Self>
    + Shr<Self, Output = Self>
{
    fn magic_number() -> Self;
    fn size() -> usize;
}

// TODO: Implement for u32
// impl MulModCompatible for u32 {
//     fn magic_number() -> Self {
//         0x8000_0000
//     }
//
//     fn size() -> usize {
//         32
//     }
// }

impl MulModCompatible for u64 {
    fn magic_number() -> Self {
        0x8000_0000_0000_0000
    }

    fn size() -> usize {
        64
    }
}

// FIXME: Hardcoded assumption that usize = u64
impl MulModCompatible for usize {
    fn magic_number() -> Self {
        0x8000_0000_0000_0000
    }

    fn size() -> usize {
        64
    }
}

/// Multiplies two numbers with a given modulus.
#[allow(clippy::many_single_char_names)]
pub fn mul_mod<T: MulModCompatible>(mut a: T, mut b: T, modulus: T) -> T {
    // https://en.wikipedia.org/wiki/Modular_arithmetic#Example_implementations

    let zero = T::default();
    let one: T = 1u8.into();
    let magic = T::magic_number();

    let mut result = zero;
    let mp2 = modulus >> one;
    a %= modulus;
    b %= modulus;

    for _ in 0..T::size() {
        result = if result > mp2 {
            (result << one) - modulus
        } else {
            result << one
        };
        if a & magic != zero {
            result += b;
        }
        if result > modulus {
            result -= modulus;
        }
        a <<= one;
    }
    result
}

/// Find t and s, such that ta + sb = gcd(p, q).
#[allow(clippy::many_single_char_names)]
pub fn bezout_coefficients(a: i64, b: i64) -> (i64, i64) {
    let mut r = (a, b);
    let mut s = (1, 0);
    let mut t = (0, 1);

    while r.1 != 0 {
        let quotient = r.0 / r.1;
        r = (r.1, r.0 - quotient * r.1);
        s = (s.1, s.0 - quotient * s.1);
        t = (t.1, t.0 - quotient * t.1);
    }

    (s.0, t.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod mul_mod {
        use super::*;

        // TODO: Implement for u32
        // #[test]
        // fn test_mul_mod_u32() {
        //     assert_eq!(mul_mod::<u32>(10, 10, 7), 2);
        //     assert_eq!(mul_mod(u32::MAX, 2, 7), 2);
        // }

        #[test]
        fn test_mul_mod_u64() {
            assert_eq!(mul_mod::<u64>(10, 10, 7), 2);
            assert_eq!(mul_mod(u64::MAX, 2, 7), 2);
        }

        #[test]
        fn test_mul_mod_usize() {
            assert_eq!(mul_mod::<usize>(10, 10, 7), 2);
            assert_eq!(mul_mod(usize::MAX, 2, 7), 2);
        }
    }

    mod gcd {
        use super::*;

        #[test]
        pub fn test() {
            assert_eq!(gcd(35, 49), 7);
        }
    }

    mod lcm {
        use super::*;

        #[test]
        pub fn test() {
            assert_eq!(lcm(35, 49), 245);
        }
    }

    mod chinese_remainder_theorem {
        use super::*;

        #[test]
        pub fn test_1() {
            assert_eq!(solve_crt((3, 1), (5, 4)), 4);
        }

        #[test]
        pub fn test_2() {
            assert_eq!(solve_crt((5, 4), (7, 6)), 34);
        }

        #[test]
        pub fn test_3() {
            assert_eq!(solve_crt((3, 1), (7, 6)), 13);
        }

        #[test]
        pub fn test_4() {
            assert_eq!(solve_crt((15, 4), (7, 6)), 34);
        }

        #[test]
        pub fn test_5() {
            assert_eq!(solve_crt((35, 34), (3, 1)), 34);
        }

        #[test]
        pub fn test_6() {
            assert_eq!(solve_crt((21, 13), (5, 4)), 34);
        }
    }

    mod bezout_coefficients {
        use super::*;

        #[test]
        pub fn test_1() {
            assert_eq!(bezout_coefficients(3, 4), (-1, 1));
        }

        #[test]
        pub fn test_2() {
            assert_eq!(bezout_coefficients(3, 5), (2, -1));
        }

        #[test]
        pub fn test_3() {
            assert_eq!(bezout_coefficients(3, 7), (-2, 1));
        }

        #[test]
        pub fn test_4() {
            assert_eq!(bezout_coefficients(5, 7), (3, -2));
        }
    }
}

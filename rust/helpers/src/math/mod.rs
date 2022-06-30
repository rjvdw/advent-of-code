//! Math.
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Rem, Sub};

pub mod polynomial;
pub mod with_gcd;

/// The absolute difference between two numbers.
pub fn abs_diff<T>(a: T, b: T) -> T
where
    T: Sub<T, Output = T> + Ord + Copy,
{
    match a.cmp(&b) {
        Ordering::Less => b.sub(a),
        _ => a.sub(b),
    }
}

/// The taxi cab distance between two 2D points.
pub fn taxi_cab_2d<T>((xa, ya): (T, T), (xb, yb): (T, T)) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Ord + Copy,
{
    abs_diff(xa, xb).add(abs_diff(ya, yb))
}

/// The taxi cab distance between two 3D points.
pub fn taxi_cab_3d<T>((xa, ya, za): (T, T, T), (xb, yb, zb): (T, T, T)) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Ord + Copy,
{
    taxi_cab_2d((xa, ya), (xb, yb)).add(abs_diff(za, zb))
}

/// The taxi cab distance between two 4D points.
pub fn taxi_cab_4d<T>((xa, ya, za, wa): (T, T, T, T), (xb, yb, zb, wb): (T, T, T, T)) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Ord + Copy,
{
    taxi_cab_3d((xa, ya, za), (xb, yb, zb)).add(abs_diff(wa, wb))
}

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

#[allow(clippy::many_single_char_names)]
fn mul_mod(mut a: u64, mut b: u64, modulus: u64) -> u64 {
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

    mod taxi_cab {
        use super::*;

        #[test]
        pub fn test_abs_diff() {
            assert_eq!(abs_diff::<i32>(7, 3), 4);
            assert_eq!(abs_diff::<i32>(3, 7), 4);
            assert_eq!(abs_diff::<i32>(-3, 7), 10);
            assert_eq!(abs_diff::<i32>(3, -7), 10);
            assert_eq!(abs_diff::<u32>(7, 3), 4);
            assert_eq!(abs_diff::<u32>(3, 7), 4);
        }

        #[test]
        pub fn test_taxi_cab_2d() {
            assert_eq!(taxi_cab_2d::<i32>((0, 0), (5, 5)), 10);
            assert_eq!(taxi_cab_2d::<i32>((-3, 2), (2, -3)), 10);
            assert_eq!(taxi_cab_2d::<u32>((0, 0), (5, 5)), 10);
        }

        #[test]
        pub fn test_taxi_cab_3d() {
            assert_eq!(taxi_cab_3d::<i32>((0, 0, 0), (5, 5, 5)), 15);
            assert_eq!(taxi_cab_3d::<i32>((-3, 2, -2), (2, -3, 3)), 15);
            assert_eq!(taxi_cab_3d::<u32>((0, 0, 5), (5, 5, 0)), 15);
        }

        #[test]
        pub fn test_taxi_cab_4d() {
            assert_eq!(taxi_cab_4d::<i32>((0, 0, 0, 0), (5, 5, 5, 5)), 20);
            assert_eq!(taxi_cab_4d::<i32>((-3, 2, -2, 2), (2, -3, 3, -3)), 20);
            assert_eq!(taxi_cab_4d::<u32>((0, 0, 5, 5), (5, 5, 0, 0)), 20);
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

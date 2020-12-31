/// Computes the greatest common divisor for numbers a and b.
#[allow(clippy::many_single_char_names)]
pub fn gcd(a: u64, b: u64) -> u64 {
    let mut ab = (a, b);
    while ab.1 != 0 {
        ab = (ab.1, ab.0 % ab.1);
    }
    ab.0
}

/// Computes the least common multiple of a and b.
#[allow(clippy::many_single_char_names)]
pub fn lcm(a: u64, b: u64) -> u64 {
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
    mul_mod(mul_mod(a, m.abs() as u64, modulus), n, modulus)
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

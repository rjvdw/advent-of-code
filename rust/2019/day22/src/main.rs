use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::ops::{Add, Rem};

use num::{BigInt, ToPrimitive};
use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::math::bezout_coefficients;

fn main() {
    let args = get_args(
        &[
            "<input file>",
            "<nr cards>",
            "<big nr cards>",
            "<big shuffle>",
        ],
        1,
    );

    let nr_cards = args[2].parse().or_exit_with(1);
    let file = File::open(&args[1]).or_exit_with(1);
    let (a, b) = parse_into_equation(file, nr_cards, 1).or_exit_with(1);
    println!("{}x + {}", a, b);
    println!("Card 2019 is at position {}.", (a * 2019 + b) % nr_cards);

    let nr_cards = args[3].parse::<i64>().or_exit_with(1);
    let shuffle_count = args[4].parse::<i64>().or_exit_with(1);
    let file = File::open(&args[1]).or_exit_with(1);
    let (a, b) = parse_into_equation(file, nr_cards, shuffle_count).or_exit_with(1);
    println!("{}x + {}", a, b);
    let card = find_card(a, b, 2020, nr_cards);
    println!("The card at position 2020 is {}.", card);
}

#[allow(clippy::many_single_char_names)]
fn find_card(a: i64, b: i64, x: i64, n: i64) -> BigInt {
    let (a_inv, _) = bezout_coefficients(a, n);
    let a_inv = BigInt::from(a_inv);
    let b = BigInt::from(b);
    let x = BigInt::from(x);
    let n = &BigInt::from(n);

    pos_rem((x - b) * a_inv, n)
}

#[allow(clippy::many_single_char_names)]
fn pos_rem<'a, T>(x: T, n: &'a T) -> T
where
    T: 'a + Rem<&'a T, Output = T> + Add<&'a T, Output = T>,
{
    ((x % n) + n) % n
}

#[allow(clippy::many_single_char_names)]
fn exp(x: i64, mut p: i64, n: i64) -> i64 {
    let mut x = BigInt::from(x);
    let n = &BigInt::from(n);
    let mut r = BigInt::from(1);

    while p > 0 {
        if p % 2 == 1 {
            r = (&r * &x) % n;
        }
        p >>= 1;
        x = (&x * &x) % n;
    }

    r.to_i64().unwrap()
}

#[allow(clippy::many_single_char_names)]
fn tail(b: i64, an: i64, a: i64, n: i64) -> i64 {
    let (div, _) = bezout_coefficients(a - 1, n);

    let b = &BigInt::from(b);
    let an = &BigInt::from(an);
    let n = &BigInt::from(n);

    let result: BigInt = (b * (an - 1) * div) % n;
    result.to_i64().unwrap()
}

#[allow(clippy::many_single_char_names)]
fn parse_into_equation<R: Read>(r: R, n: i64, p: i64) -> Result<(i64, i64), ParseError> {
    let mut a = 1;
    let mut b = 0;

    for line in BufReader::new(r).lines() {
        let line = line?;
        if line == "deal into new stack" {
            a *= -1;
            b = -b - 1;
        } else if let Some(value) = line.strip_prefix("cut ") {
            let value = value.parse::<i64>()?;
            b -= value;
        } else if let Some(value) = line.strip_prefix("deal with increment ") {
            let value = value.parse::<i64>()?;
            a *= value;
            b *= value;
        }

        a = pos_rem(a, &n);
        b = pos_rem(b, &n);
    }

    let an = exp(a, p, n);
    let b = tail(b, an, a, n);

    Ok((an, b))
}

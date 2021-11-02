use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

pub trait SplittableAndParsable {
    fn split_at_and_parse<L: FromStr, R: FromStr>(&self, mid: usize) -> Result<(L, R), ParseError>
    where
        ParseError: From<<L as FromStr>::Err> + From<<R as FromStr>::Err>;
}

impl SplittableAndParsable for &'_ str {
    fn split_at_and_parse<L: FromStr, R: FromStr>(&self, mid: usize) -> Result<(L, R), ParseError>
    where
        ParseError: From<<L as FromStr>::Err> + From<<R as FromStr>::Err>,
    {
        let (left, right) = self.split_at(mid);
        let left = left.trim().parse::<L>()?;
        let right = right[1..].trim().parse::<R>()?;
        Ok((left, right))
    }
}

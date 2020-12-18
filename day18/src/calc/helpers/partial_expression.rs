use helpers::ParseError;

pub trait PartialExpression {
    const OPEN_PAREN: char;
    const CLOSE_PAREN: char;

    fn get(&self) -> String;

    fn has_nested_expression(&self) -> bool;

    fn find(&self, ch: char) -> Option<usize>;

    fn first(a: usize, b: usize) -> usize;

    fn parse_nested_expression(
        &self,
        evaluate: fn(&str) -> Result<i64, ParseError>,
    ) -> Result<(String, i64), ParseError>;

    fn evaluate_number(
        &self,
        idx: usize,
        evaluate: fn(&str) -> Result<i64, ParseError>,
    ) -> Result<(String, i64), ParseError>;
}

use std::str::FromStr;

use crate::from_multiline_str::FromMultilineStr;

/// Helper method for parsing input using FromStr. This method is mostly useful for unit tests.
pub fn parse_input<I: FromStr>(input_lines: Vec<&str>) -> Result<Vec<I>, <I as FromStr>::Err> {
    let mut values: Vec<I> = Vec::with_capacity(input_lines.len());
    for line in input_lines {
        values.push(line.parse::<I>()?);
    }
    Ok(values)
}

/// Helper method for parsing input using FromMultilineStr. This method is mostly useful for unit
/// tests.
pub fn parse_multiline_input<I: FromMultilineStr>(
    input_lines: Vec<&str>,
) -> Result<Vec<I>, <I as FromMultilineStr>::Err> {
    let mut values = Vec::new();
    let mut record = I::new();
    let mut discard: bool = I::DISCARD_FIRST_RECORD;
    for line in input_lines {
        let line = &line.to_string();
        if I::indicates_new_record(line) {
            if !discard {
                values.push(record);
            }
            record = I::new();
            discard = false;
        }
        record.parse(line)?;
    }
    if !discard {
        values.push(record);
    }

    Ok(values)
}

/// Helper method for parsing input using FromMultilineStr. This method is mostly useful for unit
/// tests.
pub fn parse_multiline_input_as_single<I: FromMultilineStr>(
    input_lines: Vec<&str>,
) -> Result<I, <I as FromMultilineStr>::Err> {
    let mut record = I::new();
    for line in input_lines {
        let line = &line.to_string();
        record.parse(line)?;
    }

    Ok(record)
}

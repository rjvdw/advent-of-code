use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};

use rdcl_aoc_helpers::error::ParseError;

use crate::sample::Sample;

pub fn parse_input(path: &str) -> Result<(Vec<Sample>, Vec<[usize; 4]>), ParseError> {
    let mut samples: Vec<Sample> = Vec::new();
    let mut instructions: Vec<[usize; 4]> = Vec::new();
    let file = File::open(path)?;
    let mut lines = BufReader::new(file).lines();
    let mut finished_parsing_samples = false;
    while let Some(Ok(line)) = lines.next() {
        if !finished_parsing_samples {
            if let Some(sample) = parse_sample(&line, &mut lines)? {
                samples.push(sample);
                continue;
            } else {
                finished_parsing_samples = true;
            }
        }

        if finished_parsing_samples && !line.is_empty() {
            let mut instruction = [0; 4];
            for (idx, nr) in line.split_whitespace().enumerate() {
                instruction[idx] = nr.parse()?;
            }
            instructions.push(instruction);
        }
    }
    Ok((samples, instructions))
}

fn parse_sample<R: Read>(
    line: &str,
    lines: &mut Lines<BufReader<R>>,
) -> Result<Option<Sample>, ParseError> {
    let mut next_line = || {
        if let Some(l) = lines.next() {
            Ok(l?)
        } else {
            Err(ParseError::of("Parsing failed, insufficient lines."))
        }
    };

    if let Some(line1) = line.strip_prefix("Before:") {
        let line1 = line1.trim();
        let line1 = &line1[1..line1.len() - 1];
        let line2 = next_line()?;
        let line2 = line2.trim();
        let line3 = next_line()?;
        let line3 = line3
            .strip_prefix("After:")
            .ok_or_else(|| ParseError::of("Expected line to start with 'After:'."))?;
        let line3 = line3.trim();
        let line3 = &line3[1..line3.len() - 1];
        let line4 = next_line()?;

        if !line4.is_empty() {
            return Err(ParseError::of(
                "Parsing failed, encountered non-empty line where an empty line was expected.",
            ));
        }

        let mut sample = Sample {
            before: [0; 4],
            after: [0; 4],
            instruction: [0; 4],
        };

        for (idx, nr) in line1.split(", ").enumerate() {
            sample.before[idx] = nr.parse()?;
        }

        for (idx, nr) in line3.split(", ").enumerate() {
            sample.after[idx] = nr.parse()?;
        }

        for (idx, nr) in line2.split_whitespace().enumerate() {
            sample.instruction[idx] = nr.parse()?;
        }

        Ok(Some(sample))
    } else {
        Ok(None)
    }
}

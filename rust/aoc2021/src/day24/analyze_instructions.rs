use std::io;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use crate::SubRoutine;

/// The instructions can be divided into 14 subroutines that all check a single digit. Each of
/// these subroutines do pretty much the same thing:
///
///     read digit w
///     x = z % 26 + c_1
///     z /= d
///     if x != w:
///         z = z * 26 + w + c_2
///
/// (where d is either 1 or 26, and c_1 and c_2 are constants). This method extracts the relevant
/// constants from the program, so we don't need to manually execute the instructions.
///
/// Each subroutine has 18 lines. The relevant lines to read are:
/// * Line 1 always has `inp w` (this can be used to find the start of the next subroutine).
/// * Line 5 tells whether z will be divided by 1 or by 26.
/// * Line 6 contains the constant c_1.
/// * Line 16 contains the constant c_2.
pub fn analyze_instructions<I>(instructions: I) -> Result<Vec<SubRoutine>, ParseError>
where
    I: Iterator<Item = io::Result<String>>,
{
    let mut analysis = vec![];
    let mut analysis_idx = 0;
    let mut sr_idx = 0;

    for instruction in instructions {
        let instruction = instruction?;
        if instruction == "inp w" {
            analysis.push(SubRoutine::default());
            analysis_idx = analysis.len() - 1;
            sr_idx = 0;
        } else {
            let success = match sr_idx {
                4 => {
                    if let Some(v) = instruction.strip_prefix("div z ") {
                        analysis[analysis_idx].divides = v == "26";
                        true
                    } else {
                        false
                    }
                }
                5 => {
                    if let Some(v) = instruction.strip_prefix("add x ") {
                        analysis[analysis_idx].constant_1 = v.parse()?;
                        true
                    } else {
                        false
                    }
                }
                15 => {
                    if let Some(v) = instruction.strip_prefix("add y ") {
                        analysis[analysis_idx].constant_2 = v.parse()?;
                        true
                    } else {
                        false
                    }
                }
                _ => true,
            };

            if !success {
                return Err(parse_error!(
                    "Unexpected subroutine [sr:{}, #{}] - {}",
                    analysis_idx,
                    sr_idx,
                    instruction,
                ));
            }
        }
        sr_idx += 1;
    }

    Ok(analysis)
}

use std::fs::File;

use grid::Grid;
use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>", "<width>", "<height>"], 1);
    let data: String = File::open(&args[1]).read_lines(1).next().or_exit_with(1);
    let width: usize = args[2].parse().or_exit_with(1);
    let height: usize = args[3].parse().or_exit_with(1);

    println!(
        "The checksum for your image is {}.",
        checksum(&data, width, height)
    );

    let image = parse_image(&data, width, height);
    println!("The resulting image is:");
    for (idx, pixel) in image.iter().enumerate() {
        match pixel {
            1 => print!("#"),
            0 => print!("."),
            _ => unreachable!(),
        }
        if (idx + 1) % width == 0 {
            println!()
        }
    }
}

fn checksum(data: &str, width: usize, height: usize) -> usize {
    let layer_size = width * height;
    let mut result = (usize::MAX, 0);
    let mut current_layer = (0, 0, 0);
    for (idx, ch) in data.chars().enumerate() {
        match ch {
            '0' => current_layer.0 += 1,
            '1' => current_layer.1 += 1,
            '2' => current_layer.2 += 1,
            _ => {}
        }

        if (idx + 1) % layer_size == 0 {
            if current_layer.0 < result.0 {
                result = (current_layer.0, current_layer.1 * current_layer.2);
            }
            current_layer = (0, 0, 0);
        }
    }
    result.1
}

fn parse_image(data: &str, width: usize, height: usize) -> Grid<u8> {
    let mut image = Grid::from_vec(vec![2; width * height], width);

    let mut i = 0;
    let mut j = 0;
    for ch in data.chars() {
        if image[j][i] == 2 {
            match ch {
                '0' => image[j][i] = 0,
                '1' => image[j][i] = 1,
                _ => {}
            }
        }

        i += 1;
        if i == width {
            i = 0;
            j += 1;
        }
        if j == height {
            i = 0;
            j = 0;
        }
    }

    image
}

#[cfg(test)]
mod tests {
    use grid::grid;

    use super::*;

    #[test]
    fn test_checksum() {
        assert_eq!(checksum("123456789012", 3, 2), 1)
    }

    #[test]
    fn test_parse_image() {
        assert_eq!(parse_image("0222112222120000", 2, 2), grid![[0, 1][1, 0]])
    }
}

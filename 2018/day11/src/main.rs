use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

const SIZE: usize = 300;

type XY = (usize, usize);

fn main() {
    let args = get_args(&["<grid serial number>"], 1);
    let grid_serial_number = args[1].parse::<i64>().or_exit_with(1);

    let grid = get_power_grid(grid_serial_number);

    let ((x, y), power) = find_3_by_3_square(&grid);
    println!(
        "The top-left coordinate of the 3x3 square with the highest power level is {},{} (with a total power of {}).",
        x, y, power
    );
    let ((x, y), size, power) = find_square(&grid);
    println!(
        "The square with the highest power level has identifier is {},{},{} (with a total power of {}).",
        x, y, size, power
    );
}

fn get_power_level(grid_serial_number: i64, (x, y): XY) -> i64 {
    let rack_id = (x + 10) as i64;
    let t = (rack_id * (y as i64) + grid_serial_number) * rack_id;
    (t % 1000) / 100 - 5
}

fn get_power_grid(grid_serial_number: i64) -> Vec<i64> {
    let mut grid = vec![0; SIZE * SIZE];
    for x in 0..SIZE {
        for y in 0..SIZE {
            grid[y * SIZE + x] = get_power_level(grid_serial_number, (x + 1, y + 1));
        }
    }
    grid
}

fn find_3_by_3_square(grid: &[i64]) -> (XY, i64) {
    let mut squares = vec![0; (SIZE + 3) * (SIZE + 3)];

    for (idx, &cell) in grid.iter().enumerate() {
        for i in 0..3 {
            for j in 0..3 {
                squares[idx + i + j * SIZE] += cell;
            }
        }
    }

    squares
        .iter()
        .take((SIZE - 2) * (SIZE - 2))
        .enumerate()
        .max_by_key(|(_, &v)| v)
        .map(|(idx, v)| (((idx % SIZE) - 1, (idx / SIZE) - 1), *v))
        .unwrap()
}

fn find_square(grid: &[i64]) -> (XY, usize, i64) {
    let mut squares = vec![((0, 0), 0, 0); SIZE * SIZE];

    for (idx, cell) in squares.iter_mut().enumerate() {
        let x = idx % SIZE;
        let y = idx / SIZE;
        let max_size = SIZE - x.max(y) - 1;

        let mut max = (grid[idx], 1);
        let mut current = grid[idx];

        for size in 2..=max_size {
            let x_max = x + size - 1;
            let y_max = y + size - 1;
            for i in 0..size - 1 {
                let x1 = x + i;
                let y1 = y + i;
                current += grid[y_max * SIZE + x1];
                current += grid[y1 * SIZE + x_max];
            }
            current += grid[y_max * SIZE + x_max];

            if current > max.0 {
                max = (current, size);
            }
        }

        *cell = ((x + 1, y + 1), max.1, max.0);
    }

    squares.iter().max_by_key(|(_, _, v)| *v).copied().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(get_power_level(8, (3, 5)), 4);
        assert_eq!(get_power_level(57, (122, 79)), -5);
        assert_eq!(get_power_level(39, (217, 196)), 0);
        assert_eq!(get_power_level(71, (101, 153)), 4);
    }

    #[test]
    fn test_find_3_by_3_square() {
        assert_eq!(find_3_by_3_square(&get_power_grid(18)), ((33, 45), 29));
        assert_eq!(find_3_by_3_square(&get_power_grid(42)), ((21, 61), 30));
    }

    #[test]
    fn test_find_square() {
        assert_eq!(find_square(&get_power_grid(18)), ((90, 269), 16, 113));
        assert_eq!(find_square(&get_power_grid(42)), ((232, 251), 12, 119));
    }
}

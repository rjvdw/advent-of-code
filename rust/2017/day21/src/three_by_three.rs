use grid::grid;
use grid::Grid;

pub fn rotate(grid: Grid<u8>) -> Grid<u8> {
    grid![
        [grid[0][2], grid[1][2], grid[2][2]]
        [grid[0][1], grid[1][1], grid[2][1]]
        [grid[0][0], grid[1][0], grid[2][0]]
    ]
}

pub fn flip(grid: Grid<u8>) -> Grid<u8> {
    grid![
        [grid[0][0], grid[1][0], grid[2][0]]
        [grid[0][1], grid[1][1], grid[2][1]]
        [grid[0][2], grid[1][2], grid[2][2]]
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let grid = grid![
            [1, 2, 3]
            [4, 5, 6]
            [7, 8, 9]
        ];

        let grid = rotate(grid);
        assert_eq!(
            grid,
            grid![
                [3, 6, 9]
                [2, 5, 8]
                [1, 4, 7]
            ]
        );

        let grid = rotate(grid);
        assert_eq!(
            grid,
            grid![
                [9, 8, 7]
                [6, 5, 4]
                [3, 2, 1]
            ]
        );

        let grid = rotate(grid);
        assert_eq!(
            grid,
            grid![
                [7, 4, 1]
                [8, 5, 2]
                [9, 6, 3]
            ]
        );

        let grid = rotate(grid);
        assert_eq!(
            grid,
            grid![
                [1, 2, 3]
                [4, 5, 6]
                [7, 8, 9]
            ]
        );
    }

    #[test]
    fn test_flip() {
        let grid = grid![
            [1, 2, 3]
            [4, 5, 6]
            [7, 8, 9]
        ];

        let grid = flip(grid);
        assert_eq!(
            grid,
            grid![
                [1, 4, 7]
                [2, 5, 8]
                [3, 6, 9]
            ]
        );

        let grid = flip(grid);
        assert_eq!(
            grid,
            grid![
                [1, 2, 3]
                [4, 5, 6]
                [7, 8, 9]
            ]
        );
    }
}

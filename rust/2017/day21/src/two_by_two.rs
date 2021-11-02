use grid::grid;
use grid::Grid;

pub fn rotate(grid: Grid<u8>) -> Grid<u8> {
    grid![
        [grid[0][1], grid[1][1]]
        [grid[0][0], grid[1][0]]
    ]
}

pub fn flip(grid: Grid<u8>) -> Grid<u8> {
    grid![
        [grid[0][0], grid[1][0]]
        [grid[0][1], grid[1][1]]
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let grid = grid![
            [1, 2]
            [3, 4]
        ];

        let grid = rotate(grid);
        assert_eq!(
            grid,
            grid![
                [2, 4]
                [1, 3]
            ]
        );

        let grid = rotate(grid);
        assert_eq!(
            grid,
            grid![
                [4, 3]
                [2, 1]
            ]
        );

        let grid = rotate(grid);
        assert_eq!(
            grid,
            grid![
                [3, 1]
                [4, 2]
            ]
        );

        let grid = rotate(grid);
        assert_eq!(
            grid,
            grid![
                [1, 2]
                [3, 4]
            ]
        );
    }

    #[test]
    fn test_flip() {
        let grid = grid![
            [1, 2]
            [3, 4]
        ];

        let grid = flip(grid);
        assert_eq!(
            grid,
            grid![
                [1, 3]
                [2, 4]
            ]
        );

        let grid = flip(grid);
        assert_eq!(
            grid,
            grid![
                [1, 2]
                [3, 4]
            ]
        );
    }
}

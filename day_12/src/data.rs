use std::collections::HashMap;

// (row, col)
pub type Grid = HashMap<Coord, char>;
pub type Coord = (isize, isize);

// Returns (grid, start, end)
pub fn build_grid(lines: Vec<String>) -> (Grid, Coord, Coord, char) {
    let mut grid = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            grid.insert((row as isize, col as isize), char);
        }
    }

    let start = *grid.iter().find(|(_coord, &val)| val == 'S').unwrap().0;
    let end = *grid.iter().find(|(_coord, &val)| val == 'E').unwrap().0;
    let highest = *grid.values().max().unwrap();

    (grid, start, end, highest)
}

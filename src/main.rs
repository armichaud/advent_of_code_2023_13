use nalgebra::DMatrix;

const ASH: char = '.';
const ROCK: char = '#';

fn get_grids(filename: &str) -> Vec<DMatrix<char>> {
    let mut grids: Vec<DMatrix<char>> = Vec::new();
    let mut grid: DMatrix<char> = DMatrix::from_row_slice(0, 0, &[]);
    let mut row: Vec<char> = Vec::new();
    let mut col: usize = 0;
    for line in std::fs::read_to_string(filename).unwrap().lines() {
        if line.is_empty() {
            grid = DMatrix::from_row_slice(row.len() / col, col, &row);
            grids.push(grid);
            row.clear();
            col = 0;
        } else {
            row.extend(line.chars());
            col += 1;
        }
    }
    grid = DMatrix::from_row_slice(row.len() / col, col, &row);
    grids.push(grid);
    grids
}

fn get_num_of_cols_left_of_vertical_line_of_reflection(grid: &DMatrix<char>) -> Option<i32> {
    let mut n = 0;
    // TODO: Implement this function
    Some(n)
}

fn get_num_of_rows_above_horizontal_line_of_reflection(grid: &DMatrix<char>) -> Option<32> {
    let mut n = 0;
    // TODO: Implement this function
    Some(n)
}

fn solution(filename: &str) -> i32 {
    let grids: Vec<DMatrix<char>> = get_grids(filename);
    let mut sum = 0;
    for grid in grids {
        if let num_of_cols_left_of_vertical_line_of_reflection = get_num_of_cols_left_of_vertical_line_of_reflection(&grid) {
            sum += num_of_cols_left_of_vertical_line_of_reflection;
        } else if let num_of_rows_above_horizontal_line_of_reflection = get_num_of_rows_above_horizontal_line_of_reflection(&grid) {
            sum += num_of_rows_above_horizontal_line_of_reflection * 100;
        } else {
            panic!("Invalid grid: {:?}", grid);
        }
    }
    sum
}

fn main() {
    assert_eq!(solution("example.txt"), 405);
    assert_eq!(solution("input.txt"), 0);
}

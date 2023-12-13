use nalgebra::DMatrix;

// const ASH: char = '.';
// const ROCK: char = '#';

fn get_grids(filename: &str) -> Vec<DMatrix<char>> {
    let mut grids: Vec<DMatrix<char>> = Vec::new();
    let mut slice: Vec<char> = Vec::new();
    let mut rows: usize = 0;
    for line in std::fs::read_to_string(filename).unwrap().lines() {
        if line.is_empty() {
            grids.push(DMatrix::from_row_slice(rows, slice.len() / rows, &slice));
            slice.clear();
            rows = 0;
        } else {
            slice.extend(line.chars());
            rows += 1;
        }
    }
    grids.push(DMatrix::from_row_slice(rows, slice.len() / rows, &slice));
    grids
}

fn get_num_of_cols_left_of_vertical_line_of_reflection(grid: &DMatrix<char>) -> Option<i32> {
    for col in 0..grid.ncols() as i32 {
        let mut left = col;
        let mut right = col + 1;
        while left > -1 && right < grid.ncols() as i32 {
            if grid.column(left as usize) != grid.column(right as usize) {
                break;
            }
            left -= 1;
            right += 1;
        }
        if left == -1 || right == grid.ncols() as i32 {
            return Some(col as i32 - 1);
        }
    }
    None
}

fn get_num_of_rows_above_horizontal_line_of_reflection(grid: &DMatrix<char>) -> Option<i32> {
    for row in 0..grid.nrows() as i32 {
        let mut top = row;
        let mut bottom = row + 1;
        while top > -1 && bottom < grid.nrows() as i32 {
            if grid.row(top as usize) != grid.row(bottom as usize) {
                break;
            }
            top -= 1;
            bottom += 1;
        }
        if top == -1 || bottom == grid.nrows() as i32 {
            return Some(row as i32 - 1);
        }
    }
    None
}

fn solution(filename: &str) -> i32 {
    let grids: Vec<DMatrix<char>> = get_grids(filename);
    let mut sum = 0;
    for grid in grids {
        // println!("{}", grid);
        if let Some(num_of_cols_left_of_vertical_line_of_reflection) = get_num_of_cols_left_of_vertical_line_of_reflection(&grid) {
            sum += num_of_cols_left_of_vertical_line_of_reflection;
        } else if let Some(num_of_rows_above_horizontal_line_of_reflection) = get_num_of_rows_above_horizontal_line_of_reflection(&grid) {
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

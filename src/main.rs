use nalgebra::DMatrix;

const ASH: char = '.';
const ROCK: char = '#';

#[derive(Debug, PartialEq, Clone, Copy)]
enum Mirror {
    Horizontal,
    Vertical,
}

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

fn get_num_of_cols_left_of_vertical_line_of_reflection(grid: &DMatrix<char>, original_reflect_value: Option<i32>, original_reflect_axis: Option<Mirror>) -> Option<i32> {
    let original_reflect_value = original_reflect_value.unwrap_or(-1);
    let original_reflect_axis = original_reflect_axis.unwrap_or(Mirror::Horizontal);
    for col in 0..grid.ncols() as i32 - 1 {
        let mut left = col;
        let mut right = col + 1;
        while left > -1 && right < grid.ncols() as i32 {
            if grid.column(left as usize) != grid.column(right as usize) {
                break;
            }
            left -= 1;
            right += 1;
        }
        if (left == -1 || right == grid.ncols() as i32) && (original_reflect_axis == Mirror::Horizontal || original_reflect_value != col) {
            return Some(col as i32 + 1);
        }
    }
    None
}

fn get_num_of_rows_above_horizontal_line_of_reflection(grid: &DMatrix<char>, original_reflect_value: Option<i32>, original_reflect_axis: Option<Mirror>) -> Option<i32> {
    let original_reflect_value = original_reflect_value.unwrap_or(-1);
    let original_reflect_axis = original_reflect_axis.unwrap_or(Mirror::Vertical);
    for row in 0..grid.nrows() as i32 - 1 {
        let mut top = row;
        let mut bottom = row + 1;
        while top > -1 && bottom < grid.nrows() as i32 {
            if grid.row(top as usize) != grid.row(bottom as usize) {
                break;
            }
            top -= 1;
            bottom += 1;
        }
        if (top == -1 || bottom == grid.nrows() as i32) && (original_reflect_axis == Mirror::Vertical || original_reflect_value != row) {
            return Some(row as i32 + 1);
        }
    }
    None
}

fn solution(filename: &str) -> i32 {
    let grids: Vec<DMatrix<char>> = get_grids(filename);
    let mut sum = 0;
    for grid in grids {
        if let Some(n) = get_num_of_cols_left_of_vertical_line_of_reflection(&grid, None, None) {
            sum += n;
        } else if let Some(n) = get_num_of_rows_above_horizontal_line_of_reflection(&grid, None, None) {
            sum += n * 100;
        } else {
            panic!("Invalid grid: {:?}", grid);
        }
    }
    sum
}

fn swap_tiles(tile: char) -> char {
    if tile == ASH { ROCK } else { ASH }
}

fn brute_force(filename: &str) -> i32 {
    let grids: Vec<DMatrix<char>> = get_grids(filename); 
    let mut sum = 0;
    for grid in grids {
        let original_reflect_value: i32;
        let original_reflect_axis: Mirror;
        if let Some(n) = get_num_of_cols_left_of_vertical_line_of_reflection(&grid, None, None) {
            original_reflect_value = n - 1;
            original_reflect_axis = Mirror::Vertical;
        } else if let Some(n) = get_num_of_rows_above_horizontal_line_of_reflection(&grid, None, None) {
            original_reflect_value = n - 1;
            original_reflect_axis = Mirror::Horizontal;
        } else {
            panic!("Invalid grid: {:?}", grid);
        }

        let mut new_mirror_found = false;
        for i in 0..grid.nrows() {
            for j in 0..grid.ncols() {
                let mut temp_grid = grid.clone();
                temp_grid[(i, j)] = swap_tiles(grid[(i, j)]);
                if let Some(n) = get_num_of_cols_left_of_vertical_line_of_reflection(&temp_grid, Some(original_reflect_value), Some(original_reflect_axis)) {
                    sum += n;
                    new_mirror_found = true;
                    break;
                } 
                if let Some(n) = get_num_of_rows_above_horizontal_line_of_reflection(&temp_grid, Some(original_reflect_value), Some(original_reflect_axis)) {
                    sum += n * 100;
                    new_mirror_found = true;
                    break;
                } 
            }
            if new_mirror_found {
                break;
            }
        }
        if !new_mirror_found {
            panic!("No new mirror found for grid: {}", grid);
        }
    }
    sum
}

fn main() {
    println!("{}", solution("example.txt"));
    println!("{}", solution("input.txt"));
    println!("{}", brute_force("example.txt"));
    println!("{}", brute_force("input.txt")); 
}

use array2d::Array2D;
use std::ops::Range;

fn main() {
    let contents = std::fs::read_to_string("./src/bin/8_input.txt").unwrap();

    let data_vec: Array2D<u32> = Array2D::from_rows(&contents.lines().map(|line| {
        return line.chars().map(|ch: char| ch as u32 - '0' as u32).collect::<Vec<u32>>();
    }).collect::<Vec<Vec<u32>>>());

    let mut i: usize = 1;
    let mut num_visible = data_vec.num_columns() * 2 + (data_vec.num_rows() - 2) * 2;
    while i < data_vec.num_rows() -1 {
        let mut j: usize = 1;
        while j <  data_vec.num_columns() - 1 {
            // elements above
            if is_num_largest(data_vec[(i, j)], 0..i, j, true, &data_vec) 
                || is_num_largest(data_vec[(i, j)], (i+1)..data_vec.num_rows(), j, true, &data_vec) 
                || is_num_largest(data_vec[(i, j)], 0..j, i, false, &data_vec) 
                || is_num_largest(data_vec[(i, j)], (j+1)..data_vec.num_columns(), i, false, &data_vec) {
                    num_visible += 1;
            }
            j += 1;
        }
        i += 1;
    }

    let mut i: usize = 1;
    let mut scenic_scores: Vec<u32> = vec![];
    while i < data_vec.num_rows() -1 {
        let mut j: usize = 1;
        while j <  data_vec.num_columns() - 1 {
            // elements above
            scenic_scores.push(num_visible_in_direction(data_vec[(i, j)], 0..i, j, true, true, &data_vec) 
                * num_visible_in_direction(data_vec[(i, j)], (i+1)..data_vec.num_rows(), j, true, false, &data_vec) 
                * num_visible_in_direction(data_vec[(i, j)], 0..j, i, false, true, &data_vec) 
                * num_visible_in_direction(data_vec[(i, j)], (j+1)..data_vec.num_columns(), i, false, false, &data_vec)); 
                
            j += 1;
        }
        i += 1;
    }

    let res: u32 = *scenic_scores.iter().max().unwrap();

    println!("Part 1: {}", num_visible);
    println!("Part 2: {:?}", res);
}

fn is_num_largest(num: u32, range: Range<usize>, other_index: usize, is_vertical: bool, mat: &Array2D<u32>) -> bool {
    for i in range {
        if is_vertical {
            if num <= mat[(i, other_index)] {
                return false
            }
        } else {
            if num <= mat[(other_index, i)] {
                return false
            }
        }
    }
    true
}

fn num_visible_in_direction(num: u32, range: Range<usize>, other_index: usize, is_vertical: bool, iter_backwards: bool, mat: &Array2D<u32>) -> u32 {
    let mut result: u32 = 0;
    if iter_backwards {
        for i in range.rev() {
            result += 1;
            if is_vertical {
                if num <= mat[(i, other_index)] {
                    break;
                }
            } else {
                if num <= mat[(other_index, i)] {
                    break;
                }
            }
        }
    } else {
        for i in range {
            result += 1;
            if is_vertical {
                if num <= mat[(i, other_index)] {
                    break;
                }
            } else {
                if num <= mat[(other_index, i)] {
                    
                    break;
                }
            }
        }
    }
    result
}
use fraction::{Fraction, One, Zero};
use log::{debug, error, info, warn};
use std::fs;

type NumberDimension = f32;
const PRECISION: f32 = 0.00001; // Used in case of floating point shenanigans to make sure a 0.99999994 gets turned into a 1.

fn main() {
    let text: String = read_file(String::from("linear_equations_100x100.txt"));
    debug!("Debug text: {:?}", text);
    let mut matrix = load_matrix_to_variable(text);

    // Gauss - Jordan should only require n_columns steps
    for un in 0..matrix.len() {
        // Select viable pivot
        let (x, y) = select_pivot(&matrix);

        // Divide pivot line by the value
        let multiplier = matrix[x][y];
        multiply_matrix_line(&mut matrix, x, multiplier.recip());

        // Loop through lines
        for j in 0..matrix.len() {
            // Skip pivot line
            if j == x {
                continue;
            }

            // Get element in same column as pivot
            let pivot_sort_of = matrix[j][y];
            let mut pivot_row = matrix[x].clone();

            // Multiply pivot row with the value
            multiply_line(&mut pivot_row, -pivot_sort_of);
            // Sum the current line with the modified pivot row
            sum_matrix_line(&mut matrix, j, pivot_row);
        }
    }

    save_solution(&mut matrix);
}

fn is_solution_complete(matrix: &Vec<Vec<NumberDimension>>) -> bool {
    // Check if every column (except the last one) has only one 1 and the rest zeroes
    let mut one_found: bool;
    // We can probably assume every row has the same lenght
    for i in 0..matrix[0].len() - 1 {
        one_found = false;
        for j in 0..matrix.len() {
            // Check if the number is != 0
            if matrix[j][i] != 0.0 && matrix[j][i] != 1.0 {
                return false;
            }
            // If we find a one, set the found variable. If found again, we are not done yet
            if matrix[j][i] == 1.0 {
                if one_found {
                    return false;
                } else {
                    one_found = true;
                }
            }
        }
    }
    true
}

fn select_pivot(matrix: &Vec<Vec<NumberDimension>>) -> (usize, usize) {
    // Try every point (unless it's a zero)
    for mut j in 0..matrix.len() {
        // Also skip the last colums, as it's not part of A
        for i in 0..matrix[j].len() - 1 {
            // First check -> Is the number zero?
            if matrix[j][i] != /*Fraction::zero()*/ 0.0 {
                //println!("Pivot {},{} -> {} passed first check", j, i, &matrix[j][i]);
                // Check two: is the number one, AND the rest of the column zeroes?
                // If this fails, we probably should move one row down. Forcefully.
                if matrix[j][i] != /*Fraction::one()*/ 1.0
                    || !validate_column_elements(matrix, i, vec![0.0, 1.0])
                {
                    //println!("Pivot {},{} -> {} passed second check", j, i, &matrix[j][i]);
                    return (j, i);
                } else {
                    j += 1;
                }
            }
        }
    }
    // Should probably return an exception or something. TODO
    (0, 0)
}

fn validate_column_elements(
    matrix: &Vec<Vec<NumberDimension>>,
    column_index: usize,
    wanted: Vec<NumberDimension>,
) -> bool {
    for i in 0..matrix.len() {
        if !wanted.contains(&matrix[i][column_index]) {
            return false;
        }
    }
    true
}

fn read_file(filepath: String) -> String {
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    contents
}

fn load_matrix_to_variable(input_matrix: String) -> Vec<Vec<NumberDimension>> {
    let mut matrix_values: Vec<Vec<NumberDimension>> = vec![vec![]];

    let mut rowindex = 0;

    for line in input_matrix.lines() {
        let split_iterator: std::str::Split<'_, char> = line.split(';');
        let mut row: Vec<NumberDimension> = Vec::new();
        debug!("Adding row {}", rowindex);

        for split in split_iterator {
            // Skip empty elements. Should only happen after the last ;
            if split.trim().is_empty() {
                continue;
            }

            let tmp2 = split.trim().parse::<NumberDimension>();

            if tmp2.is_err() {
                error!("Error adding value: {:?}", tmp2);
            } else {
                let unwrapval = tmp2.unwrap(); // Ottiene il valore del parse
                debug!("Adding {:?}", unwrapval);
                row.push(unwrapval);
            }
        }
        rowindex += 1;
        matrix_values.push(row);
    }

    // There is an extra empty array at the start. Remove it
    matrix_values.remove(0);

    matrix_values
}

fn pretty_print_matrix(matrix: &Vec<Vec<NumberDimension>>) {
    for row in matrix {
        for element in row {
            print!("{:.2}\t", element);
        }
        println!();
    }
}

fn multiply_matrix_line(
    matrix: &mut Vec<Vec<NumberDimension>>,
    line: usize,
    multiplier: NumberDimension,
) {
    matrix[line] = matrix[line]
        .iter()
        .map(|value| {
            if (value.abs() - multiplier.recip().abs()).abs() <= PRECISION {
                1.0
            } else {
                value * multiplier
            }
        })
        .collect::<Vec<NumberDimension>>();
}

fn multiply_line(line: &mut Vec<NumberDimension>, multiplier: NumberDimension) {
    line.iter_mut().for_each(|value| *value *= multiplier);
}

fn sum_matrix_line(
    matrix: &mut Vec<Vec<NumberDimension>>,
    line: usize,
    sum_line: Vec<NumberDimension>,
) {
    matrix[line] = matrix[line]
        .iter()
        .zip(sum_line.iter())
        .map(|(a, b)| a + b)
        .collect();
}

fn save_solution(matrix: &mut Vec<Vec<NumberDimension>>){
    // We only care about the final column
    let b_index = matrix[0].len() - 1;

    let mut string_to_file = String::from("");

    // Teoretically, it should always be a triangular matrix.
    for i in 0..matrix.len(){
        string_to_file += &format!("X{} = {}\n", i, matrix[i][b_index]);
    }

    // Save to file
    fs::write("rust_solution.txt", string_to_file).expect("Unable to write file");
}
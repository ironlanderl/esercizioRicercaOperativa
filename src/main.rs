use fraction::{Fraction, One, Zero};
use log::{debug, error, info, warn};
use std::fs;

type NumberDimension = f32;

fn main() {
    let text: String = read_file(String::from("test.txt"));
    debug!("Debug text: {:?}", text);
    let mut matrix = load_matrix_to_variable(text);
    println!("Input Matrix: ");
    pretty_print_matrix(&matrix);

    // Select viable pivot
    let (x, y) = select_pivot(&matrix);
    println!("Coordinate pivot -> X: {}, Y: {}", x, y);

    // Divide pivot line by the value
    let multiplier = matrix[x][y];
    multiply_matrix_line(&mut matrix, x, multiplier.recip());
    println!("Line {} divided by {}", x, multiplier.recip());
    pretty_print_matrix(&matrix);

    // Loop through lines
    for j in 0..matrix.len() {
        // Skip pivot line
        if j == x {
            continue;
        }

        // Get element in same column as pivot
        let pivot_sort_of = matrix[j][y];
        let mut pivot_row = matrix[x].clone();

        // Mutliply pivot row with the value
        multiply_line(&mut pivot_row, -pivot_sort_of);
        // Sum the current line with the modified pivot row
        sum_matrix_line(&mut matrix, j, pivot_row);
    }
    println!("Matrix after reduction: ");
    pretty_print_matrix(&matrix);

    // New pivot
    let (x, y) = select_pivot(&matrix);
    println!("Coordinate pivot -> X: {}, Y: {}", x, y);
}

fn select_pivot(matrix: &Vec<Vec<NumberDimension>>) -> (usize, usize) {
    // Try every point (unless it's a zero)
    for j in 0..matrix.len() {
        for i in 0..matrix[j].len() {
            // First check -> Is the number zero?
            if matrix[j][i] != /*Fraction::zero()*/ 0.0 {
                println!("Pivot {},{} -> {} passed first check", j, i, &matrix[j][i]);
                // Check two: is the number one, AND the rest of the column zeroes?
                if matrix[j][i] != /*Fraction::one()*/ 1.0
                    || !check_column(matrix, i, j, /* Fraction::zero()*/ 0.0)
                {
                    println!("Pivot {},{} -> {} passed second check", j, i, &matrix[j][i]);
                    return (j, i);
                }
            }
        }
    }
    // Should probably return an exception or something. TODO
    (0, 0)
}

fn check_column(
    matrix: &Vec<Vec<NumberDimension>>,
    column_index: usize,
    row_to_skip: usize,
    wanted: NumberDimension,
) -> bool {
    for i in 0..matrix.len() {
        if i == row_to_skip {
            continue;
        }
        if matrix[i][column_index] != wanted {
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
            print!("{}\t", element);
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
        .map(|value| value * multiplier)
        .collect::<Vec<NumberDimension>>();
}

fn multiply_line(line: &mut Vec<NumberDimension>, multiplier: NumberDimension) {
    for i in 0..line.len() {
        line[i] *= multiplier;
    }
}

fn sum_matrixes(matrix: &mut Vec<Vec<NumberDimension>>, sum_matrix: Vec<Vec<NumberDimension>>) {
    for j in 0..matrix.len() {
        for i in 0..matrix[j].len() {
            matrix[j][i] += sum_matrix[j][i];
        }
    }
}

fn sum_matrix_line(
    matrix: &mut Vec<Vec<NumberDimension>>,
    line: usize,
    sum_line: Vec<NumberDimension>,
) {
    for i in 0..matrix[line].len() {
        matrix[line][i] += sum_line[i];
    }
}

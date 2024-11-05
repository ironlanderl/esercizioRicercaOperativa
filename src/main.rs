use std::fs;
use log::{info, warn, debug, error};

fn main() {
    let text: String = read_file(String::from("test.txt"));
    debug!("Debug text: {:?}", text);
    let mut matrix = load_matrix_to_variable(text);
    //println!("Debug Matrix: {:?}", matrix);
    println!("Input Matrix: ");
    pretty_print_matrix(matrix.clone());
    matrix[0] = matrix[0].iter().map(|value| { value * 2.0 }).collect::<Vec<f32>>();
    println!("Double first line: ");
    pretty_print_matrix(matrix.clone());
}

fn read_file(filepath: String) -> String {
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    contents
}

fn load_matrix_to_variable(input_matrix: String) -> Vec<Vec<f32>> {
    let mut matrix_values: Vec<Vec<f32>> = vec![vec![]];

    let mut rowindex = 0;

    for line in input_matrix.lines() {
        let split_iterator: std::str::Split<'_, char> = line.split(';');
        let mut row: Vec<f32> = Vec::new();
        debug!("Adding row {}", rowindex);

        for split in split_iterator {
            // Skip empty elements. Should only happen after the last ;
            if split.trim().is_empty() {
                continue;
            }

            let tmp2 = split.trim().parse::<f32>();

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

fn pretty_print_matrix(matrix: Vec<Vec<f32>>) {
    for row in matrix {
        for element in row {
            print!("{}\t", element);
        }
        println!();
    }
}



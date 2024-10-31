use vector2d::Vector2D;
use std::fs;

fn unused1(){
    let mut test: Vec<Vec<i32>> = vec![vec![0]];
    test[5][4] = 2;
}

fn main() {
    let text: String = read_file(String::from("test.txt"));
    println!("Debug: {:?}", text);
}

fn read_file(filepath: String) -> String{
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    contents
}

fn load_matrix_to_variable(matrix: String) -> Vec<Vec<i32>>{
    
}
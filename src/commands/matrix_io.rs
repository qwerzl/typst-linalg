use serde::{Serialize, Deserialize};
use serde_json::{from_slice, to_vec};

use super::super::types::DefaultMatrix;

#[derive(Serialize, Deserialize, Debug)]
struct CreateMatrixInput {
    x: i32,
    y: i32,
    vec: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateMatrixOutput {
    matrix: DefaultMatrix
}

pub fn create_matrix(arg: &[u8]) -> Vec<u8> {
    let input: CreateMatrixInput = from_slice(arg).unwrap();
    let matrix = na::DMatrix::from_row_slice(input.x as usize, input.y as usize, &input.vec);

    let result = CreateMatrixOutput { matrix };
    to_vec(&result).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
struct ToMatInput {
    matrix: DefaultMatrix,
}

#[derive(Serialize, Deserialize, Debug)]
struct ToMatOutput {
    command: String,
}

pub fn to_mat(arg: &[u8]) -> Vec<u8> {
  let input: ToMatInput = from_slice(arg).unwrap();
  let matrix = input.matrix;
  let mut command = "mat(".to_string();
  for i in 0..matrix.nrows() {
    for j in 0..matrix.ncols() {
    command.push_str(&matrix[(i, j)].to_string());
    if j < matrix.ncols() - 1 {
        command.push_str(", ");
    }
  }
  if i < matrix.nrows() - 1 {
      command.push(';');
  }
  }
  command.push_str(";)");
  let result = ToMatOutput { command };
  
  to_vec(&result).unwrap()
}

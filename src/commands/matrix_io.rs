use serde::{Serialize, Deserialize};
use serde_json::{from_slice, to_vec};

use super::super::types::DefaultMatrix;
use super::super::utils::f64_to_string;

/*
  Creates a matrix from a vector of f64 values
  The matrix is created with the given dimensions

  Input:
    x: i32 - number of rows
    y: i32 - number of columns
    vec: Vec<f64> - vector of f64 values to create the matrix
*/
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

/*
  Converts a matrix to a string
  The matrix is converted to a string that can be used in the mat() function in Octave

  Input:
    matrix: DefaultMatrix - matrix to convert
    truncate?: (usize, usize) - number of rows and columns to show for the matrix. 
                                truncated items are replaced with elipsis.
                                use (0, 0) to show all items
*/
#[derive(Serialize, Deserialize, Debug)]
struct ToMatInput {
  matrix: DefaultMatrix,

  #[serde(default = "default_truncate")]
  truncate: (isize, isize),

  #[serde(default = "default_precision")]
  precision: isize,
}

fn default_truncate() -> (isize, isize) {
  (-1, -1)
}

fn default_precision() -> isize {
  6
}

#[derive(Serialize, Deserialize, Debug)]
struct ToMatOutput {
  command: String,
}

pub fn to_mat(arg: &[u8]) -> Vec<u8> {
  let input: ToMatInput = from_slice(arg).unwrap();
  let mut matrix = input.matrix;

  // Truncate the matrix
  let mut is_truncated = (false, false);
  {
    if input.truncate.0 > 0 {
      let d = (input.truncate.0 - 1) as usize;
      let i = matrix.nrows() - input.truncate.0 as usize;
      matrix = matrix.remove_rows(d, i);
      is_truncated.0 = true;
    }
    if input.truncate.1 > 0 {
      let d = (input.truncate.1 - 1) as usize;
      let i = matrix.ncols() - input.truncate.1 as usize;
      matrix = matrix.remove_columns(d, i);
      is_truncated.1 = true;
    }
  }

  // Produce the command
  let mut command = String::from("mat(");
  {
    for i in 0..matrix.nrows() {
      for j in 0..matrix.ncols() {

        command.push_str(&f64_to_string(
          matrix[(i, j)], input.precision
        ));

        if j < matrix.ncols() - 1 {
            command.push_str(", ");
        }
        if j == matrix.ncols() - 2 && is_truncated.1 {
            command.push_str("dots.h, ");
        }
      }
      if i < matrix.nrows() - 1 {
          command.push_str("; ");
      }
      if i == matrix.nrows() - 2 && is_truncated.0 {
        let mut dots = ["dots.v"].repeat(matrix.ncols() + is_truncated.1 as usize);
        if is_truncated.1 && input.truncate.0 > 1 && input.truncate.1 > 1 {
          dots[matrix.ncols() - 1] = "dots.down";
        }
        command.push_str(dots.join(",").as_str());
        command.push_str("; ")
      }
    }
    command.push(')');
  }
  let result = ToMatOutput { command };
  
  to_vec(&result).unwrap()
}

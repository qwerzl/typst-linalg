use na::VecStorage;
use serde::{Serialize, Deserialize};
use serde_json::{from_slice, to_vec};

use super::super::types::DefaultMatrix;

use nalgebra::{Matrix, Const, Dyn};

type RowMatrix = Matrix<f64, Const<1>, Dyn, VecStorage<f64, Const<1>, Dyn>>;
type ColumnMatrix = Matrix<f64, Dyn, Const<1>, VecStorage<f64, Dyn, Const<1>>>;

#[derive(Serialize, Deserialize, Debug)]
struct Input {
  matrix: DefaultMatrix,
}

#[derive(Serialize, Deserialize, Debug)]
struct IntoRowsOutput {
  matrices: Vec<RowMatrix>,
}

pub fn into_rows(arg: &[u8]) -> Vec<u8> {
  let input: Input = from_slice(arg).unwrap();

  let mut matrices: Vec<RowMatrix> = Vec::new();
  for n in 0..input.matrix.nrows() {
    let row = input.matrix.row(n).into_owned();
    matrices.push(row);
  }

  let result = IntoRowsOutput { matrices };
  to_vec(&result).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
struct IntoColumnsOutput {
  matrices: Vec<ColumnMatrix>,
}

pub fn into_columns(arg: &[u8]) -> Vec<u8> {
  let input: Input = from_slice(arg).unwrap();

  let mut matrices: Vec<ColumnMatrix> = Vec::new();
  for n in 0..input.matrix.ncols() {
    let column = input.matrix.column(n).into_owned();
    matrices.push(column);
  }

  let result = IntoColumnsOutput { matrices };
  to_vec(&result).unwrap()
} 
use serde::{Serialize, Deserialize};
use serde_json::{from_slice, to_vec};

use super::super::types::DefaultMatrix;

/*
  add/subtract/multiply/divide.

  Input:
    matrices: Vec<DefaultMatrix> - matrices to add/subtract/multiply
  Output:
    matrix: DefaultMatrix - result of the operation
*/
#[derive(Serialize, Deserialize, Debug)]
struct Input {
  matrix: DefaultMatrix,
  scalar: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct Output {
  matrix: DefaultMatrix,
}

pub fn add_scalar(arg: &[u8]) -> Vec<u8> {
  let input: Input = from_slice(arg).unwrap();
  let mut matrix = input.matrix;

  for e in matrix.iter_mut() {
    *e += input.scalar
  }

  let result = Output { matrix };
  to_vec(&result).unwrap()
}

pub fn subtract_scalar(arg: &[u8]) -> Vec<u8> {
  let input: Input = from_slice(arg).unwrap();
  let mut matrix = input.matrix;

  for e in matrix.iter_mut() {
    *e -= input.scalar
  }

  let result = Output { matrix };
  to_vec(&result).unwrap()
}

pub fn multiply_scalar(arg: &[u8]) -> Vec<u8> {
  let input: Input = from_slice(arg).unwrap();
  let mut matrix = input.matrix;

  for e in matrix.iter_mut() {
    *e *= input.scalar
  }

  let result = Output { matrix };
  to_vec(&result).unwrap()
}

pub fn divide_scalar(arg: &[u8]) -> Vec<u8> {
  let input: Input = from_slice(arg).unwrap();
  let mut matrix = input.matrix;

  for e in matrix.iter_mut() {
    *e /= input.scalar
  }

  let result = Output { matrix };
  to_vec(&result).unwrap()
}
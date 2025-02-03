use serde::{Serialize, Deserialize};
use serde_json::{from_slice, to_vec};

use super::super::types::DefaultMatrix;

/*
  The following functions are used to perform +/-/product.
  The input is a JSON object with the following structure:
  {
    "matrices": [matrix1, matrix2, ...]
  }
  The output is a JSON object with the following structure:
  {
    "matrix": result
  }
*/
#[derive(Serialize, Deserialize, Debug)]
struct Input {
  matrices: Vec<DefaultMatrix>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Output {
  matrix: DefaultMatrix,
}

pub fn add(arg: &[u8]) -> Vec<u8> {
  let input: Input = from_slice(arg).unwrap();

  let result = Output { matrix: input.matrices.iter().sum() };
  to_vec(&result).unwrap()
}

pub fn subtract(arg: &[u8]) -> Vec<u8> {
  let input: Input = from_slice(arg).unwrap();

  let mut res = input.matrices[0].clone();
  for m in input.matrices.iter().skip(1) {
    res -= m;
  }

  let result = Output { matrix: res };
  to_vec(&result).unwrap()
}

pub fn multiply(arg: &[u8]) -> Vec<u8> {
  let input: Input = from_slice(arg).unwrap();

  let mut res = input.matrices[0].clone();
  for m in input.matrices.iter().skip(1) {
    res *= m;
  }

  let result = Output { matrix: res };
  to_vec(&result).unwrap()
}

/*
  Modifies a matrix at a specific location

  Input:
    matrix: DefaultMatrix - matrix to modify
    location: (usize, usize) - location to modify
    value: f64 - value to set at the location
  
  Output:
    matrix: DefaultMatrix - modified matrix
*/
#[derive(Serialize, Deserialize, Debug)]
struct ModifyInput {
  matrix: DefaultMatrix,
  location: (usize, usize),
  value: f64,
}

pub fn modify(arg: &[u8]) -> Vec<u8> {
  let input: ModifyInput = from_slice(arg).unwrap();

  let mut matrix = input.matrix.clone();
  matrix[input.location] = input.value;
  let result = Output { matrix };
  to_vec(&result).unwrap()
}

/*
  Transposes a matrix

  Input:
    matrix: DefaultMatrix - matrix to transpose
  
  Output:
    matrix
*/

#[derive(Serialize, Deserialize, Debug)]
struct TransposeInput {
  matrix: DefaultMatrix,
}

pub fn transpose(arg: &[u8]) -> Vec<u8> {
  let input: TransposeInput = from_slice(arg).unwrap();

  let result = Output { matrix: input.matrix.transpose() };
  to_vec(&result).unwrap()
}
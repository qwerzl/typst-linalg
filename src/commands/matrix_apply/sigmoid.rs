use serde::{Serialize, Deserialize};
use serde_json::{from_slice, to_vec};

use super::super::super::types::DefaultMatrix;

#[derive(Serialize, Deserialize, Debug)]
struct Input {
  matrix: DefaultMatrix
}

#[derive(Serialize, Deserialize, Debug)]
struct Output {
  matrix: DefaultMatrix
}

fn _sigmoid(v: f64) -> f64 {
  1.0 / (1.0 + f64::exp(-v))
}

pub fn sigmoid(arg: &[u8]) -> Vec<u8> {
  let input: Input = from_slice(arg).unwrap();
  let matrix = input.matrix;

  let result = Output { matrix: matrix.map(_sigmoid) };
  to_vec(&result).unwrap()
}
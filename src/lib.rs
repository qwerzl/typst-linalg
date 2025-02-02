use commands::*;
use wasm_minimal_protocol::*;

mod types;
mod commands;

extern crate nalgebra as na;

initiate_protocol!();

#[wasm_func]
pub fn create_matrix(arg: &[u8]) -> Vec<u8> {
    matrix_io::create_matrix(arg)
}

#[wasm_func]
pub fn to_mat(arg: &[u8]) -> Vec<u8> {
    matrix_io::to_mat(arg)
}

#[wasm_func]
pub fn add(arg: &[u8]) -> Vec<u8> {
    matrix_operations::add(arg)
}

#[wasm_func]
pub fn subtract(arg: &[u8]) -> Vec<u8> {
    matrix_operations::subtract(arg)
}

#[wasm_func]
pub fn multiply(arg: &[u8]) -> Vec<u8> {
    matrix_operations::multiply(arg)
}

#[wasm_func]
pub fn modify(arg: &[u8]) -> Vec<u8> {
    matrix_operations::modify(arg)
}

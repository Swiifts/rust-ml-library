use colored::*;

#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

pub enum MatrixData {
    Vector(Vec<f64>),
    Scalar(f64),
    Matrix(Matrix),
    Usize(usize),
}

impl MatrixData{
    pub fn initialize(self, rows: usize, cols: usize) -> Vec<f64> {
        match self {
            MatrixData::Vector(vec) => {
                if vec.len() != rows*cols {
                    panic!("{}", format!("ERROR in initialize() (file: {}, line: {}). Invalid size for matrix initialization. Expected {}x{} elements but got {}.", file!(), line!(), rows, cols, vec.len()).red().bold())
                }
                vec
            }
            MatrixData::Scalar(val) => vec![val; rows*cols],
            MatrixData::Matrix(mat) => {
                if mat.rows != rows || mat.cols != cols {
                    panic!("{}", format!("ERROR in initialize() (file: {}, line: {}). Invalid size for matrix initialization. Expected {}x{} but got {}x{}.", file!(), line!(), rows, cols, mat.rows, mat.cols).red().bold())
                }
                mat.data
            }       
            MatrixData::Usize(val) => vec![val as f64; rows*cols],     
        }
    }
}





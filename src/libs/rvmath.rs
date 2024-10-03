use std::fmt; 
use std::ops::{Add, Mul, Sub, Div};
use rand::Rng;

#[derive(Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}
pub enum MatrixData {
    Vector(Vec<f64>),
    Scalar(f64),
    Matrix(Matrix),
}

impl MatrixData{
    fn initialize(self, rows: usize, cols: usize) -> Vec<f64> {
        match self {
            MatrixData::Vector(vec) => {
                if vec.len() != rows*cols {
                    panic!("Invalid size for matrix initialization. Expected {}x{} elements but got {}.", rows, cols, vec.len());
                }
                vec
            }
            MatrixData::Scalar(val) => vec![val; rows*cols],
            MatrixData::Matrix(mat) => mat.data,            
        }
    }
}

impl From<Vec<f64>> for MatrixData {
    fn from(value: Vec<f64>) -> Self {
        MatrixData::Vector(value)
    }
}

impl From<f64> for MatrixData {
    fn from(val: f64) -> Self {
        MatrixData::Scalar(val)
    }
}

impl Matrix {

    pub fn new<T: Into<MatrixData>>(rows: usize, cols: usize, data: T) -> Matrix {
        let data = data.into().initialize(rows, cols);
        Matrix { rows, cols, data }
    }

    pub fn rand(rows: usize, cols: usize) -> Matrix {
        let mut rng = rand::thread_rng();
        let data = (0..rows * cols).map(|_| rng.gen_range(0.0..1.0)).collect();
        Matrix { rows, cols, data }
    }
    
    // TODO Identity matrix
}

impl Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, scalar: f64) -> Matrix {
        let mut result_matrix = Matrix::new(self.rows, self.cols, scalar);
        for i in 0..(self.rows*self.cols) {
            result_matrix.data[i] *= self.data[i];
        }
        result_matrix
    }
}

impl Div<f64> for Matrix {
    type Output = Matrix;

    fn div(self, scalar: f64) -> Matrix {
        let mut result_matrix = Matrix::new(self.rows, self.cols, 0.0);
        for i in 0..(self.rows*self.cols) {
            result_matrix.data[i] = self.data[i] / scalar;
        }
        result_matrix
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix{

        if self.cols != other.rows { panic!("Matrix dimentions do not match {}x{} and {}x{}",self.rows,self.cols,other.rows,other.cols) }

        let mut result_matrix: Matrix = Matrix::new(self.rows, other.cols, 0.0);

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i * self.cols + k] * other.data[ k * other.cols + j];
                }
                result_matrix.data[i * other.cols + j] = sum;
            }
        }

        result_matrix
    }
}

impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, other: Matrix) -> Matrix {
        
        if (self.rows != other.rows) || (self.cols != other.cols) { panic!("Matrix dimentions do not match {}x{} and {}x{}",self.rows,self.cols,other.rows,other.cols) }
        
        let mut result_matrix: Matrix = Matrix::new(self.rows, self.cols, 0.0);

        for i in 0..(self.rows*self.cols) {
            result_matrix.data[i]=self.data[i]-other.data[i];
        }

        result_matrix
    }
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Matrix {
        
        if (self.rows != other.rows) || (self.cols != other.cols) { panic!("Matrix dimentions do not match {}x{} and {}x{}",self.rows,self.cols,other.rows,other.cols) }
        
        let mut result_matrix: Matrix = Matrix::new(self.rows, self.cols, 0.0);

        for i in 0..(self.rows*self.cols) {
            result_matrix.data[i]=self.data[i]+other.data[i];
        }

        result_matrix
    }
}

// TODO Redo Display into more standart math format
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self.data[row * self.cols + col])?;
                if col < self.cols - 1 {
                    write!(f, "\t")?; 
                }
            }
            writeln!(f)?; 
        }
        Ok(())
    }
}

pub fn dot(a: Matrix, b: Matrix) -> f64 {
    if (a.rows != b.rows) || (a.cols != b.cols) { panic!("Matrix dimentions do not match {}x{} and {}x{}",a.rows,a.cols,b.rows,b.cols) }
    let mut dot_product = 0.0;
    for i in 0..(a.rows*a.cols) {
        dot_product += a.data[i] * b.data[i];
    }
    dot_product
}

pub fn mean(arr: &[Matrix]) -> Matrix {
    if arr.is_empty() { panic!("You have not provided any valid matrices!") }
    if arr.len() == 1 { return arr[0].clone(); }
    for i in 1..arr.len() {
        if arr[0].rows != arr[i].rows || arr[0].cols != arr[i].cols { panic!("Matrix dimentions do not match {}x{} and {}x{}",arr[0].rows,arr[0].cols,arr[i].rows,arr[i].cols) }
    }
    let mut result_matrix: Matrix = Matrix::new(arr[0].rows, arr[0].cols, 0.0);

    for i in 0..arr.len() {
        for j in 0..(arr[0].rows * arr[0].cols) {
            result_matrix.data[j] += arr[i].data[j];
        }
    }

    result_matrix / arr.len() as f64

}

// TODO Transpose

// TODO Determinant

// TODO Inverse

// TODO Eigenvalues 

// TODO Eigenvectors

// TODO "Map" on Matrix

// TODO Norms


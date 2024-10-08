use std::fmt; 
use std::ops::{Add, Mul, Sub, Div};
use rand::Rng;
use colored::*;
use std::collections::HashMap;
use ordered_float::OrderedFloat;
use super::structures::{Matrix, MatrixData};

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

impl From<usize> for MatrixData {
    fn from(val: usize) -> Self {
        MatrixData::Usize(val)
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
    
    pub fn identity(size: usize) -> Matrix {
        let mut result_matrix: Matrix = Matrix::new(size, size, 0);
        
        for i in 0..size {
            result_matrix.data[i + size*i] = 1.0;
        }
        result_matrix
    }

    pub fn transpose(&self) -> Matrix {
        let mut result_matrix: Matrix = Matrix::new(self.cols, self.rows, 0);
        
        for row in 0..self.rows {
            for col in 0..self.cols {
                result_matrix.data[col * self.rows + row] = self.data[row * self.cols + col];
            }
        }

        result_matrix
    }

    pub fn map(&self, f: fn(f64) -> f64) -> Matrix {
        let mut result_matrix: Matrix = Matrix::new(self.rows, self.cols, 0.0);
        for i in 0..(self.rows*self.cols) {
            result_matrix.data[i] = f(self.data[i])
        }
        result_matrix
    }

    pub fn reshape(self, rows:usize, cols:usize) -> Matrix {
        if self.rows*self.cols != rows*cols { panic!("{}", format!("ERROR in reshape() (file: {}, line: {}). Dimensions mismatch: {}x{} cannot be reshaped into {}x{}.", file!(), line!(), self.rows, self.cols, rows, cols).red().bold()); }
        Matrix {rows, cols, data: self.data}
    }

    pub fn sum(&self) -> f64{
        let mut sum: f64 = 0.0;
        for i in 0..(self.rows*self.cols) {
            sum += self.data[i];
        }
        sum
    }
    
    pub fn magnitude(&self) -> f64 {
        if self.rows != 1 && self.cols != 1 {panic!("{}", format!("ERROR in magnitude() (file: {}, line: {}). Matrix dimensions {}x{} must be a vector (*x1 or 1x*).", file!(), line!(), self.rows, self.cols).red().bold())}
        self.map(|x: f64| x.powi(2)).sum().sqrt()
    }

    pub fn median(&self) -> f64 {
        let mut arr = self.data.clone();
        arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if self.data.len() % 2 == 0 {
            let median: f64 = (arr[arr.len()/2-1]+arr[arr.len()-2])/2.0;
            median
        } else {
            let median: f64 = arr[arr.len()/2];
            median
        }
    }

    pub fn percentile(&self, p: f64) -> f64 {
        if p < 0.0 || p > 1.0 {panic!("{}", format!("ERROR in percentile() (file: {}, line: {}). Percentile must be in range 0.0-1.0, whereas provided {}", file!(), line!(), p).red().bold())}
        let mut arr = self.data.clone();
        arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let pointer: f64 = (arr.len() as f64 - 1.0) * p;
    
        if pointer % 1.0 != 0.0 {
            let percentile = (arr[pointer.ceil() as usize] + arr[pointer.floor() as usize]) / 2.0;
            percentile
        } else {
            let index = pointer as usize;
            arr[index]
        }
    }
    
    pub fn mode(&self) -> Vec<f64> {
        let mut values: HashMap<OrderedFloat<f64>, usize> = HashMap::new();
        for &value in &self.data {
            *values.entry(OrderedFloat(value)).or_insert(1) += 1;
        }
        let max_count = values.values().cloned().max().unwrap_or(0);
        let mut modes = Vec::new();
        for (&OrderedFloat(value), &count) in &values {
            if count == max_count {
                modes.push(value);
            }
        }
    
        modes
    }
        
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

        if self.cols != other.rows { panic!("{}", format!("ERROR in mul() (file: {}, line: {}). Matrix dimensions do not match: {}x{} and {}x{}.", file!(), line!(), self.rows, self.cols, other.rows, other.cols).red().bold()) }

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
        
        if (self.rows != other.rows) || (self.cols != other.cols) { panic!("{}", format!("ERROR in sub() (file: {}, line: {}). Matrix dimensions do not match: {}x{} and {}x{}.", file!(), line!(), self.rows, self.cols, other.rows, other.cols).red().bold()) }
        
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
        
        if (self.rows != other.rows) || (self.cols != other.cols) { panic!("{}", format!("ERROR in add() (file: {}, line: {}). Matrix dimensions do not match: {}x{} and {}x{}.", file!(), line!(), self.rows, self.cols, other.rows, other.cols).red().bold()) }
        
        let mut result_matrix: Matrix = Matrix::new(self.rows, self.cols, 0.0);

        for i in 0..(self.rows*self.cols) {
            result_matrix.data[i]=self.data[i]+other.data[i];
        }

        result_matrix
    }
}

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
    if (a.rows != b.rows) || (a.cols != b.cols) { panic!("{}", format!("ERROR in dot() (file: {}, line: {}). Matrix dimensions do not match: {}x{} and {}x{}.", file!(), line!(), a.rows, a.cols, b.rows, b.cols).red().bold()) }
    let mut dot_product = 0.0;
    for i in 0..(a.rows*a.cols) {
        dot_product += a.data[i] * b.data[i];
    }
    dot_product
}

pub fn mean(arr: &[Matrix]) -> Matrix {
    if arr.is_empty() { panic!("{}", format!("ERROR in mean() (file: {}, line: {}). You have not provided any valid matrices!", file!(), line!()).red().bold()) }
    if arr.len() == 1 { return arr[0].clone(); }
    for i in 1..arr.len() {
        if arr[0].rows != arr[i].rows || arr[0].cols != arr[i].cols { panic!("{}", format!("ERROR in mean() (file: {}, line: {}). Matrix dimensions do not match: {}x{} and {}x{}.", file!(), line!(), arr[0].rows, arr[0].cols, arr[i].rows, arr[i].cols).red().bold()) }
    }
    let mut result_matrix: Matrix = Matrix::new(arr[0].rows, arr[0].cols, 0.0);

    for i in 0..arr.len() {
        for j in 0..(arr[0].rows * arr[0].cols) {
            result_matrix.data[j] += arr[i].data[j];
        }
    }

    result_matrix / arr.len() as f64

}

pub fn distance(a: Matrix, b: Matrix) -> f64{
    (a - b).magnitude()
}

// TODO Inverse

// TODO Determinant

// TODO Eigenvalues 

// TODO Eigenvectors

// TODO Norms

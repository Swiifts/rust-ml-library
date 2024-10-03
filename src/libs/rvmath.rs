use std::fmt; 
use std::ops::Mul;
use rand::Rng;

pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {

    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Matrix {
        
        if data.len() != rows * cols { panic!("Invalid size for matrix initialisation\nRows: {}; Cols: {}; Data len: {}", rows, cols, data.len()); }
    
        Matrix { rows, cols, data }
    }

    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix { rows, cols, data: vec![0.0; rows*cols] }
    }

    pub fn ones(rows: usize, cols: usize) -> Matrix {
        Matrix { rows, cols, data: vec![1.0; rows*cols] }
    }

    pub fn rand(rows: usize, cols: usize) -> Matrix {
        let mut rng = rand::thread_rng();
        let data = (0..rows * cols).map(|_| rng.gen_range(0.0..1.0)).collect();
        Matrix { rows, cols, data }
    }

}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix{

        if self.cols != other.rows { panic!("Matrix dimentions do not match for multiplications\nRows: {}; Columns: {}", self.rows, other.cols); }

        let mut result_matrix = Matrix::zeros(self.rows, other.cols);

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

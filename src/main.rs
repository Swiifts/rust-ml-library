mod libs {
    pub mod rvmath;
}
use libs::rvmath::{self, structures::Matrix};

fn main() {
    let matrix1 = Matrix::new(1, 4, vec![3.0,4.0,3.0,4.0]);
    let matrix2 = Matrix::new(2, 2, vec![3.0,4.0,3.0,4.0]);

    let result = rvmath::matrix::distance(matrix1, matrix2.reshape(2,3));

    println!("Resulting Matrix:\n{}", result);
}

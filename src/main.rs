#[path ="libs/rvmath.rs"] mod rvmath;

use crate::rvmath::Matrix;

fn main() {
    let matrix1 = Matrix::new(2, 2, vec![222.0, 2.0, 3.0, 4.0]);
    let matrix2 = Matrix::rand(2, 2);

    let result = matrix1 * matrix2;

    println!("Resulting Matrix:\n{}", result);
}
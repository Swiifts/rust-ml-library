#[path ="libs/rvmath.rs"] mod rvmath;

use crate::rvmath::Matrix;

fn main() {
    let matrix1 = Matrix::new(4, 2, vec![222.0, 2.0, 3.0, 4.0, 111.0, 2.0, 3.0, 4.0]);
    let matrix2 = Matrix::new(4, 1, vec![111.0, 2.0, 3.0, 4.0]);
    let scalar = 01;
    println!("{}", scalar);

    let result: Matrix = Matrix::identity(4);

    println!("Resulting Matrix:\n{}", result);
}
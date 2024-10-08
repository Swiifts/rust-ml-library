mod libs {
    pub mod rvmath;
}
use libs::rvmath::{self, structures::Matrix};

fn main() {
    let arr: Matrix = Matrix::new(3, 3, vec![1.0,2.0,3.0,1.0,2.0,3.0,5.0,5.0,5.0]);
    println!("{:?}",arr.sum_col());

}

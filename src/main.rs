mod libs {
    pub mod rvmath;
}
use libs::rvmath::{self, structures::Matrix};

fn main() {
    let arr: Matrix = Matrix::new(1,5, vec![1.0,2.0,2.0,3.0,4.0]);
    println!("{:?}",arr.mode());

}

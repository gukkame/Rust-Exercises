#[derive(Debug)]
#[derive(PartialEq)]
pub struct Matrix((i32, i32),(i32, i32));
pub fn transpose(m: Matrix) -> Matrix {
    let (a, b) = m.0;
    let (c, d) = m.1;
    Matrix((a,c),(b,d))
}
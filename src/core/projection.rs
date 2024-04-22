use std::f32::consts::PI;

use crate::Matrix;
use std::fs::File;
use std::io::Write;

pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
    let f = 1. / (fov * 0.5 * PI / 180.).tan();
    let nf = 1.  / (far - near);

    Matrix::from(&[
        &[f / ratio, 0., 0., 0.],
        &[0., f, 0., 0.],
        &[0., 0., -far * nf, -1.],
        &[0., 0., -far * near * nf, 0.],
    ])
}

pub fn save_matrix(matrix: &Matrix<f32>, filename: &str) {

    let mut file = File::create(filename).unwrap();
    for row in 0..matrix.shape().0 {
        for col in 0..matrix.shape().1 {
            if col != matrix.shape().0 - 1 {
                write!(file, "{}, ", matrix[row][col]).unwrap();
            } else {
                write!(file, "{}", matrix[row][col]).unwrap();
            }
        }
        writeln!(file).unwrap();
    }
}
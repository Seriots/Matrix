#![allow(dead_code)]
use std::{fmt::Display, ops::{AddAssign, Index, IndexMut, MulAssign, Range, Sub, SubAssign}};

use crate::{utils::IntoF32, Vector};
use crate::utils::Fma;

use crate::core::linear_interpolation::Lerp;


#[derive(Clone, Debug, Default)]
pub struct Matrix<K> {
    pub matrix: Vec<Vec<K>>,
}

impl<K: Clone + Default + Fma + IntoF32> Matrix<K> {
    pub fn from(matrix: Vec<Vec<K>>) -> Self {
        for i in 0..matrix.len() {
            if matrix[i].len() != matrix[0].len() {
                panic!("All rows must have the same length");
            }
        }
        Self { matrix }
    }

    pub fn shape(&self) -> (usize, usize) {
        return (self[0].len(), self.matrix.len())
    }

    pub fn is_square(&self) -> bool {
        return self[0].len() == self.matrix.len()
    }

    pub fn to_vector(&self) -> Vector<K> {
        let mut vector: Vec<K> = Vec::new();

        for i in 0..self.matrix.len() {
            vector.append(&mut self[i].clone());
        }
        return Vector::from(vector);
    }
}

impl<K: Clone + Default + Fma + IntoF32 + AddAssign + SubAssign + MulAssign> Matrix<K> {
    pub fn add(&mut self, v: &Matrix<K>) {
        if self.shape() != v.shape() {
            panic!("Size are different")
        }
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                self[i][j] += v[i][j].clone();
            }
        }
    }

    pub fn sub(&mut self, v: &Matrix<K>) {
        if self.shape() != v.shape() {
            panic!("Size are different")
        }
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                self[i][j] -= v[i][j].clone();
            }
        }
    }

    pub fn scl(&mut self, a: K) {
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                self[i][j] *= a.clone();
            }
        }
    }
}

impl<K: Clone + Default + Fma + IntoF32 + Sub<Output = K> + From<f32>> Lerp for Matrix<K> {
    fn lerp(self, v: Self, t: f32) -> Self {
       let mut res = self.clone();
       let shape = self.shape();

        for i in 0..shape.0 {
            for j in 0..shape.1 {
                res[i][j] = Fma::fma(v[i][j].clone() - self[i][j].clone(), t.into(), self[i][j].clone()); 
            }
        }
        return res;
    }
}

// ------------------------------------------------------------------------- //
/// Implementing Index and IndexMut for Matrix => accssor []

impl<K> Index<usize> for Matrix<K> {
    type Output = Vec<K>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}

impl<K> IndexMut<usize> for Matrix<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}

impl<K> Index<(usize, usize)> for Matrix<K> {
    type Output = K;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.matrix[index.0][index.1]
    }
}

impl<K> IndexMut<(usize, usize)> for Matrix<K> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[index.0][index.1]
    }
}

impl<K> Index<Range<usize>> for Matrix<K> {
    type Output = [Vec<K>];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.matrix[range]
    }
}

impl<K> IndexMut<Range<usize>> for Matrix<K> {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        &mut self.matrix[range]
    }
}

impl<K> Index<(Range<usize>, Range<usize>)> for Matrix<K> {
    type Output = [Vec<K>];

    fn index(&self, range:(Range<usize>, Range<usize>)) -> &Self::Output {
        &self.matrix[range.0][range.1]
    }
}

impl<K> IndexMut<(Range<usize>, Range<usize>)> for Matrix<K> {
    fn index_mut(&mut self, range: (Range<usize>, Range<usize>)) -> &mut Self::Output {
        &mut self.matrix[range.0][range.1]
    }
}

// ------------------------------------------------------------------------- //
/// Implementing Display for matrix

impl<K: Clone + Default + Fma + IntoF32 + Display> Display for Matrix<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut all_rows: String = String::new();
        let mut max_size: Vec<usize> = Vec::new();
        let shape = self.shape();

        for i in 0..shape.0 {
            for j in 0..shape.1 {
                if i == 0 { max_size.push(0); } 
                if self[i][j].to_string().len() > max_size[j] {
                    max_size[j] = self[i][j].to_string().len();
                }
            }
        }

        for i in 0..shape.0 {
            all_rows += "[ ";
            for j in 0..shape.1 {
                all_rows += &format!("{:>max$} ", &self[i][j], max=max_size[j]);
            }
            all_rows += "]";
            if i != shape.0 - 1 {
                all_rows += "\n";
            }
        } 
        write!(f, "{}", all_rows)
    }
}
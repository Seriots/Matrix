#![allow(dead_code)]
use std::{default, fmt::{Debug, Display}, ops::{AddAssign, Index, IndexMut, MulAssign, Range, Sub, SubAssign}};

use crate::{utils::IntoF32, Vector};
use crate::utils::Fma;

use crate::core::linear_interpolation::Lerp;


#[derive(Clone, Debug, Default)]
pub struct Matrix<K> {
    pub matrix: Vec<Vec<K>>,
}

impl<K: Clone + Default + Fma + IntoF32 + Sub<Output = K>> PartialEq for Matrix<K> {
    fn eq(&self, other: &Self) -> bool {
        if self.shape() != other.shape() {
            return false;
        }
        for i in 0..self.shape().1 {
            for j in 0..self.shape().0 {
                let a = (self[i][j].clone() - other[i][j].clone()).into_f32();
                if a > 1e-6 || a < -1e-6 {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<K: Clone + Default + Fma + IntoF32> Matrix<K> {
    pub fn from(matrix: &[&[K]]) -> Self {
        let mut new_matrix: Vec<Vec<K>> = Vec::new();

        for i in 0..matrix.len() {
            if matrix[i].len() != matrix[0].len() {
                panic!("All rows must have the same length");
            }
            new_matrix.push(matrix[i].to_vec());
        }
        Self { matrix: new_matrix }
    }

    pub fn from_vec(matrix: Vec<Vec<K>>) -> Self {
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
        return Vector::from_vec(vector);
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

    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        let base_shape = self.shape();

        if vec.size() != base_shape.0 {
            panic!("Vector must have the same size than the number of columns")
        }

        let mut new = Vector::from_vec(vec![K::default(); base_shape.1]);
        for i in 0..base_shape.0 {
            for j in 0..base_shape.1 {
                new[j].sfma(vec[i].clone(), self[j][i].clone());
            }
        }

        return new;
    }

    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        let base_shape = self.shape();
        let mut new: Matrix<K> = Matrix::from_vec(vec![vec![K::default(); mat.shape().0]; base_shape.1]);
        
        if mat.shape().1 != base_shape.0 {
            panic!("Argument must have the same amount of rows than the amout of columns")
        }
        for k in 0..mat.shape().0 {
            for i in 0..base_shape.0 {
                for j in 0..base_shape.1 {
                   new[j][k].sfma(mat[i][k].clone(), self[j][i].clone()); 
                }
            }
        }
        return new;
    }

    pub fn trace(&self) -> K {
        let mut tr: K = K::default();
        if !self.is_square() {
            panic!("This matrix is not a square")
        }
        for i in 0..self.shape().0 {
            tr += self[i][i].clone();
        }
        return tr;
    }

    pub fn transpose(&self) -> Matrix<K> {
        
        let mut new: Matrix<K> = Matrix::from_vec(vec![vec![K::default(); self.shape().1]; self.shape().0]);

        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                new[i][j] = self[j][i].clone();
            }
        }
        return new;
    }

    pub fn row_echelon(&self) -> Matrix<K> {
        
        return Matrix::default();
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

        for i in 0..shape.1 {
            for j in 0..shape.0 {
                if i == 0 { max_size.push(0); }
                if self[i][j].to_string().len() > max_size[j] {
                    max_size[j] = self[i][j].to_string().len();
                }
            }
        }

        for i in 0..shape.1 {
            all_rows += "[ ";
            for j in 0..shape.0 {
                all_rows += &format!("{:>max$} ", &self[i][j], max=max_size[j]);
            }
            all_rows += "]";
            if i != shape.1 - 1 {
                all_rows += "\n";
            }
        } 
        write!(f, "{}", all_rows)
    }
}
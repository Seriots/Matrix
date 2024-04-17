#![allow(dead_code)]

use std::{cmp::max, fmt::Display, ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign}};

use num_traits::pow;

use crate::Matrix;
use crate::fma::Fma;
use crate::basics::linear_interpolation::Lerp;

#[derive(Debug, Clone, Default)]
pub struct Vector<K> {
    pub vector: Vec<K>,
}

impl<K: Clone + Default + Fma> Vector<K> {
    pub fn from(array: Vec<K>) -> Self {
        Self { vector: array }
    }

    pub fn size(&self) -> usize {
        return self.vector.len();
    }

    pub fn to_matrix(&self, shape: (usize, usize)) -> Matrix<K> {
        let mut matrix: Vec<Vec<K>> = Vec::new();
        
        if shape.0 == 0 || shape.1 == 0 {
            panic!("Shape can't be 0")
        }
        if shape.0 * shape.1 != self.size() {
           panic!("This vector can't be converted in this shape") 
        }
        for i in 0..shape.1 {
            matrix.push(Vec::new());
            matrix[i].append(&mut self.vector[(i*shape.1)..(i*shape.1 + shape.0)].to_vec());
        }
        return Matrix::from(matrix)
    }
}


impl<K: Clone + Default + Fma + AddAssign + SubAssign + MulAssign> Vector<K>
where f32: From<K> {
    pub fn add(&mut self, v: &Vector<K>) {
        if self.size() != v.size() {
            panic!("Size are different")
        }
        for i in 0..self.size() {
            self[i] += v[i].clone();
        }
    }

    pub fn sub(&mut self, v: &Vector<K>) {
        if self.size() != v.size() {
            panic!("Size are different")
        }
        for i in 0..self.size() {
            self[i] -= v[i].clone();
        }
    }

    pub fn scl(&mut self, a: K) {
        for i in 0..self.size() {
            self[i] *= a.clone(); 
        }
    }

    pub fn dot(&self, v: Self) -> K {
        let mut result: K = K::default();

        for i in 0..self.size() {
            result.sfma(self[i].clone(), v[i].clone());
        }
        return result;
    }

    //Taxicab norm or Manhattan norm (||v||1)
    pub fn norm_1(&self) -> f32 {
        let mut res = f32::default();
        
        for i in 0..self.size() {
            let v: f32 = self[i].clone().into();
            res += v.max(-v);
        }

        return res;
    }

    //Euclidean norm (||v||2)
    pub fn norm(&self) -> f32 {
        let mut res = K::default();

        for i in 0..self.size() {
            res.sfma(self[i].clone(), self[i].clone());
        }
        return f32::powf(res.into(), 0.5);
    }

    // Supremum norm (||v||∞)
    pub fn norm_inf(&self) -> f32 {

        let mut res = f32::default();
        
        for i in 0..self.size() {
            let v: f32 = self[i].clone().into();
            res = res.max(v.max(-v));
        }

        return res;
    }

}

impl<K: Clone + Default + Fma + Sub<Output = K> + Add<Output = K> + Mul<Output = K> + From<f32>> Lerp for Vector<K> {
    fn lerp(self, v: Self, t: f32) -> Self {
       let mut res = self.clone();

        for i in 0..self.size() {
            //(v - u) * t + u
            res[i] = Fma::fma(v[i].clone() - self[i].clone(), t.into(), self[i].clone()); 
        }
        return res;
    }
}

impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vector[index]
    }
}

impl<K> IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vector[index]
    }
}

impl<K: Clone + Default + Fma + Display> Display for Vector<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut all: String = String::new();
        let mut max_size = 0;
        for i in 0..self.size() {
            if  self[i].to_string().len() > max_size {
                max_size = self[i].to_string().len()
            }
        }
        for i in 0..self.size() {
            all += &format!("[{:>max$}]", self[i], max=max_size);
            if i != self.size() -1 {
                all += "\n";
            }

        }
        write!(f, "{}", all)
    }
}


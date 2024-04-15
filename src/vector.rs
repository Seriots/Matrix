#![allow(dead_code)]

use std::{fmt::Display, ops::{AddAssign, MulAssign, SubAssign}};

use crate::matrix::Matrix;


#[derive(Debug, Clone)]
pub struct Vector<K> {
    pub vector: Vec<K>,
}

impl<K: Clone> Vector<K> {
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

impl<K: AddAssign + SubAssign + MulAssign + Clone> Vector<K> {
    pub fn add(&mut self, v: &Vector<K>) {
        if self.size() != v.size() {
            panic!("Size are different")
        }
        for i in 0..self.size() {
            self.vector[i] += v.vector[i].clone();
        }
    }

    pub fn sub(&mut self, v: &Vector<K>) {
        if self.size() != v.size() {
            panic!("Size are different")
        }
        for i in 0..self.size() {
            self.vector[i] -= v.vector[i].clone();
        }
    }

    pub fn scl(&mut self, a: K) {
        for i in 0..self.size() {
            self.vector[i] *= a.clone(); 
        }
    }
}

impl<K: Display + Clone> Display for Vector<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut all: String = String::new();
        let mut max_size = 0;
        for i in 0..self.size() {
            if  self.vector[i].to_string().len() > max_size {
                max_size = self.vector[i].to_string().len()
            }
        }
        for i in 0..self.size() {
            all += &format!("[{:>max$}]", self.vector[i], max=max_size);
            if i != self.size() -1 {
                all += "\n";
            }

        }
        write!(f, "{}", all)
    }
}
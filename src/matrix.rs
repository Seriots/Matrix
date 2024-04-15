#![allow(dead_code)]
use std::{fmt::Display, ops::{AddAssign, MulAssign, SubAssign}};

use crate::vector::Vector;


#[derive(Clone, Debug)]
pub struct Matrix<K> {
    pub matrix: Vec<Vec<K>>,
}

impl<K: Clone> Matrix<K> {
    pub fn from(matrix: Vec<Vec<K>>) -> Self {
        for i in 0..matrix.len() {
            if matrix[i].len() != matrix[0].len() {
                panic!("All rows must have the same length");
            }
        }
        Self { matrix }
    }

    pub fn shape(&self) -> (usize, usize) {
        return (self.matrix[0].len(), self.matrix.len())
    }

    pub fn is_square(&self) -> bool {
        return self.matrix[0].len() == self.matrix.len()
    }

    pub fn to_vector(&self) -> Vector<K> {
        let mut vector: Vec<K> = Vec::new();

        for i in 0..self.matrix.len() {
            vector.append(&mut self.matrix[i].clone());
        }
        return Vector::from(vector);
    }
}

impl<K: AddAssign + SubAssign + MulAssign + Clone> Matrix<K> {
    pub fn add(&mut self, v: &Matrix<K>) {
        if self.shape() != v.shape() {
            panic!("Size are different")
        }
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                self.matrix[i][j] += v.matrix[i][j].clone();
            }
        }
    }

    pub fn sub(&mut self, v: &Matrix<K>) {
        if self.shape() != v.shape() {
            panic!("Size are different")
        }
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                self.matrix[i][j] -= v.matrix[i][j].clone();
            }
        }
    }

    pub fn scl(&mut self, a: K) {
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                self.matrix[i][j] *= a.clone();
            }
        }
    }
}

impl<K: Display + Clone> Display for Matrix<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut all_rows: String = String::new();
        let mut max_size: Vec<usize> = Vec::new();

        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                if i == 0 { max_size.push(0); } 
                if self.matrix[i][j].to_string().len() > max_size[j] {
                    max_size[j] = self.matrix[i][j].to_string().len();
                }
            }
        }

        for i in 0..self.matrix.len() {
            all_rows += "[ ";
            for j in 0..self.matrix[i].len() {
                all_rows += &format!("{:>max$} ", &self.matrix[i][j], max=max_size[j]);
            }
            all_rows += "]";
            if i != self.matrix.len() - 1 {
                all_rows += "\n";
            }
        } 
        write!(f, "{}", all_rows)
    }
}
#![allow(dead_code)]

use crate::{basics::Vector, fma::Fma};

pub fn linear_combination<K: Clone + Default + Fma>(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
    let mut mu = Vector::from(vec![K::default(); u[0].size()]);

    for i in 0..u.len() {
        for j in 0..u[0].size() {
            mu.vector[j].sfma(u[i].vector[j].clone(), coefs[i].clone());
        }  
    } 
    return mu;
}

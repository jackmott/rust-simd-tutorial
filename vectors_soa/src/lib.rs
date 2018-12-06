use std::arch::x86_64::*;

pub struct Vectors3 {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub z: Vec<f32>,
}

pub struct Entities {
    pub name: Vec<String>,
    pub pos: Vectors3,
    pub v: Vectors3,
    pub mass: Vec<f32>,
    pub elasticity: Vec<f32>,
    pub strength: Vec<f32>
}

pub trait Vector {
    fn add(&mut self, b: &Self);   
    fn norm(&mut self);
    fn simd_add(&mut self, b: &Self);    
    fn simd_norm(&mut self);
}

impl Vector for Vectors3 {
    fn add(&mut self, b: &Vectors3) {
        for i in 0..self.x.len() {
            self.x[i] += b.x[i];
            self.y[i] += b.y[i];
            self.z[i] += b.z[i];
        }
    }
 
    fn norm(&mut self) {
        for i in 0..self.x.len() {
            let len =
                1.0/(self.x[i] * self.x[i] + self.y[i] * self.y[i] + self.z[i] * self.z[i]).sqrt();
            self.x[i] *= len;
            self.y[i] *= len;
            self.z[i] *= len;
        }
    }

    fn simd_add(&mut self, b: &Vectors3) {
       
    }


    fn simd_norm(&mut self) {
       
    }
}

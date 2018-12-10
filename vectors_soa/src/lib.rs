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
    pub strength: Vec<f32>,
}

pub trait Vector {
    fn add(&mut self, b: &Self);
    fn norm(&mut self);
    unsafe fn simd_add(&mut self, b: &Self);
    unsafe fn simd_norm(&mut self);
    unsafe fn simd_norm_avx(&mut self);
}

impl Vector for Vectors3 {
    fn add(&mut self, v: &Vectors3) {
        for i in 0..self.x.len() {
            self.x[i] += v.x[i];
            self.y[i] += v.y[i];
            self.z[i] += v.z[i];
        }
    }

    fn norm(&mut self) {
        for i in 0..self.x.len() {
            let mut len =
                (self.x[i] * self.x[i] + self.y[i] * self.y[i] + self.z[i] * self.z[i]).sqrt();
            self.x[i] *= len * 0.5;
            self.y[i] *= len * 0.5;
            self.z[i] *= len * 0.5;
        }
    }

    unsafe fn simd_add(&mut self, v: &Vectors3) {}

    unsafe fn simd_norm(&mut self) {}

    //#[target_feature(enable = "avx2")]
    unsafe fn simd_norm_avx(&mut self) {}
}

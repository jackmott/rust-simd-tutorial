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
    fn clamp(&mut self, min: f32);
    unsafe fn sse_add(&mut self, b: &Self);
    unsafe fn sse_norm(&mut self);
    unsafe fn sse_clamp(&mut self, min: f32);
    unsafe fn avx_clamp(&mut self, min: f32);
}

impl Vector for Vectors3 {
    fn add(&mut self, v: &Vectors3) {
        for i in 0..self.x.len() {
            self.x[i] += v.x[i];
            self.y[i] += v.y[i];
            self.z[i] += v.z[i];
        }
    }

    unsafe fn sse_add(&mut self, v: &Vectors3) {}

    fn norm(&mut self) {
        for i in 0..self.x.len() {
            let len =
                (self.x[i] * self.x[i] + self.y[i] * self.y[i] + self.z[i] * self.z[i]).sqrt();
            self.x[i] /= len;
            self.y[i] /= len;
            self.z[i] /= len;
        }
    }

    fn clamp(&mut self, min: f32) {
        for i in 0..self.x.len() {
            let mut len =
                (self.x[i] * self.x[i] + self.y[i] * self.y[i] + self.z[i] * self.z[i]).sqrt();
            if len < min {
                len = (1.0/len)*min;
                self.x[i] *= len;
                self.y[i] *= len;
                self.z[i] *= len;
            }
        }
    }

    unsafe fn sse_norm(&mut self) {
       
    }

    unsafe fn sse_clamp(&mut self, min: f32) {
               
    }

    //#[target_feature(enable = "avx2")]
    unsafe fn avx_clamp(&mut self, min: f32) {
    
    }
}

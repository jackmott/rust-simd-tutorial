use std::arch::x86_64::*;
use std::mem::*;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Entity {
    pub name: String,
    pub pos: Vector3,
    pub v: Vector3,
    pub mass: f32,
    pub elasticity: f32,
    pub strength: f32,
}

pub trait Vector {
    fn add(&mut self, b: &Self);
    fn norm(&mut self);
    unsafe fn sse_add(&mut self, b: &Self);
    unsafe fn sse_norm(&mut self);
}

impl Vector for Vector3 {
    fn add(&mut self, v: &Vector3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }

    fn norm(&mut self) {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    unsafe fn sse_add(&mut self, v: &Vector3) {
        let a = _mm_set_ps(self.x,self.y,self.z,0.0);
        let b = _mm_set_ps(v.x,v.y,v.z,0.0);
        let result = _mm_add_ps(a,b);
        let result_array = transmute::<__m128,[f32;4]>(result);
        self.x = result_array[3];
        self.y = result_array[2];
        self.z = result_array[1];
    
    }

    unsafe fn sse_norm(&mut self) {

    }
}

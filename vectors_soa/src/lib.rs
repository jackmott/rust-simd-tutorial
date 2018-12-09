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

    unsafe fn simd_add(&mut self, v: &Vectors3) {
        for i in (0..self.x.len()).step_by(4) {
            let ax = _mm_loadu_ps(self.x.get_unchecked(i));
            let bx = _mm_loadu_ps(v.x.get_unchecked(i));

            let ay = _mm_loadu_ps(self.y.get_unchecked(i));
            let by = _mm_loadu_ps(v.y.get_unchecked(i));

            let az = _mm_loadu_ps(self.z.get_unchecked(i));
            let bz = _mm_loadu_ps(v.z.get_unchecked(i));
            
            _mm_storeu_ps(self.x.get_unchecked_mut(i),_mm_add_ps(ax,bx));
            _mm_storeu_ps(self.y.get_unchecked_mut(i),_mm_add_ps(ay,by));
            _mm_storeu_ps(self.z.get_unchecked_mut(i),_mm_add_ps(az,bz));
    
        }
      
    }

    unsafe fn simd_norm(&mut self) {
      for i in (0..self.x.len())
    }

    //#[target_feature(enable = "avx2")]
    unsafe fn simd_norm_avx(&mut self) {

    }
}

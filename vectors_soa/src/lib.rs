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
}

pub trait Vector {
    fn add(&mut self, b: &Self);
    fn mul(&mut self, scale: f32);
    fn norm(&mut self);
    fn simd_add(&mut self, b: &Self);
    fn simd_mul(&mut self, scale: f32);
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

    fn mul(&mut self, b: f32) {
        for i in 0..self.x.len() {
            self.x[i] *= b;
            self.y[i] *= b;
            self.z[i] *= b;
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
        unsafe {
            let mut i = 0;
            while i < self.x.len() {
                let selfx = _mm_loadu_ps(self.x.get_unchecked(i) as *const f32);
                let selfy = _mm_loadu_ps(self.y.get_unchecked(i) as *const f32);
                let selfz = _mm_loadu_ps(self.z.get_unchecked(i) as *const f32);

                let bx = _mm_loadu_ps(b.x.get_unchecked(i) as *const f32);
                let by = _mm_loadu_ps(b.y.get_unchecked(i) as *const f32);
                let bz = _mm_loadu_ps(b.z.get_unchecked(i) as *const f32);

                _mm_storeu_ps(
                    self.x.get_unchecked_mut(i) as *mut f32,
                    _mm_add_ps(selfx, bx),
                );
                _mm_storeu_ps(
                    self.y.get_unchecked_mut(i) as *mut f32,
                    _mm_add_ps(selfy, by),
                );
                _mm_storeu_ps(
                    self.z.get_unchecked_mut(i) as *mut f32,
                    _mm_add_ps(selfz, bz),
                );

                i += 4;
            }
        }
    }

    fn simd_mul(&mut self, scale: f32) {
        unsafe {
            let mut i = 0;
            let scale_simd = _mm_set1_ps(scale);
            while i < self.x.len() {
                let selfx = _mm_loadu_ps(&self.x[i] as *const f32);
                let selfy = _mm_loadu_ps(&self.y[i] as *const f32);
                let selfz = _mm_loadu_ps(&self.z[i] as *const f32);

                _mm_storeu_ps(&mut self.x[i] as *mut f32, _mm_mul_ps(selfx, scale_simd));
                _mm_storeu_ps(&mut self.y[i] as *mut f32, _mm_mul_ps(selfy, scale_simd));
                _mm_storeu_ps(&mut self.z[i] as *mut f32, _mm_mul_ps(selfz, scale_simd));

                i += 4;
            }
        }
    }

    fn simd_norm(&mut self) {
        unsafe {
            let mut i = 0;
            while i < self.x.len() {
                let mut selfx = _mm_loadu_ps(self.x.get_unchecked(i) as *const f32);
                let mut selfy = _mm_loadu_ps(self.y.get_unchecked(i) as *const f32);
                let mut selfz = _mm_loadu_ps(self.z.get_unchecked(i) as *const f32);

                let len = _mm_rsqrt_ps(_mm_add_ps(
                    _mm_mul_ps(selfz, selfz),
                    _mm_add_ps(_mm_mul_ps(selfx, selfx), _mm_mul_ps(selfy, selfy)),
                ));

                selfx = _mm_mul_ps(selfx, len);
                selfy = _mm_mul_ps(selfy, len);
                selfz = _mm_mul_ps(selfz, len);

                _mm_storeu_ps(self.x.get_unchecked_mut(i) as *mut f32, selfx);
                _mm_storeu_ps(self.y.get_unchecked_mut(i) as *mut f32, selfy);
                _mm_storeu_ps(self.z.get_unchecked_mut(i) as *mut f32, selfz);

                i += 4;
            }
        }
    }
}

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
            if len > 2.0 {
                len = 1.0 / len;
                self.x[i] *= len * 0.5;
                self.y[i] *= len * 0.5;
                self.z[i] *= len * 0.5;
            }
        }
    }

    unsafe fn simd_add(&mut self, v: &Vectors3) {
        for i in (0..self.x.len()).step_by(4) {
            let x = _mm_loadu_ps(self.x.get_unchecked(i));
            let y = _mm_loadu_ps(self.y.get_unchecked(i));
            let z = _mm_loadu_ps(self.z.get_unchecked(i));

            let vx = _mm_loadu_ps(v.x.get_unchecked(i));
            let vy = _mm_loadu_ps(v.y.get_unchecked(i));
            let vz = _mm_loadu_ps(v.z.get_unchecked(i));
        }
    }

    unsafe fn simd_norm(&mut self) {
        for i in (0..self.x.len()).step_by(4) {
            let mut x = _mm_loadu_ps(self.x.get_unchecked(i));
            let mut y = _mm_loadu_ps(self.y.get_unchecked(i));
            let mut z = _mm_loadu_ps(self.z.get_unchecked(i));

            let len = _mm_rsqrt_ps(_mm_add_ps(
                _mm_add_ps(_mm_mul_ps(x, x), _mm_mul_ps(y, y)),
                _mm_mul_ps(z, z),
            ));

            let cond = _mm_cmpgt_ps(len, _mm_set1_ps(2.0));

            let true_result = _mm_mul_ps(len, _mm_set1_ps(0.5));
            let false_result = _mm_set1_ps(1.0);
            let result = _mm_or_ps(
                _mm_and_ps(cond, true_result),
                _mm_andnot_ps(cond, false_result),
            );

            x = _mm_mul_ps(x, result);
            y = _mm_mul_ps(y, result);
            z = _mm_mul_ps(z, result);

            _mm_storeu_ps(self.x.get_unchecked_mut(i), x);
            _mm_storeu_ps(self.y.get_unchecked_mut(i), y);
            _mm_storeu_ps(self.z.get_unchecked_mut(i), z);
        }
    }

    #[target_feature(enable = "avx2")]
    unsafe fn simd_norm_avx(&mut self) {
        for i in (0..self.x.len()).step_by(8) {
            let mut x = _mm256_loadu_ps(self.x.get_unchecked(i));
            let mut y = _mm256_loadu_ps(self.y.get_unchecked(i));
            let mut z = _mm256_loadu_ps(self.z.get_unchecked(i));

            let len = _mm256_rsqrt_ps(_mm256_add_ps(
                _mm256_add_ps(_mm256_mul_ps(x, x), _mm256_mul_ps(y, y)),
                _mm256_mul_ps(z, z),
            ));

            let cond = _mm256_cmp_ps(len, _mm256_set1_ps(2.0), _CMP_GT_OQ);

            let true_result = _mm256_mul_ps(len, _mm256_set1_ps(0.5));
            let false_result = _mm256_set1_ps(1.0);
            let result = _mm256_blendv_ps(true_result,false_result,cond);
            
            x = _mm256_mul_ps(x, result);
            y = _mm256_mul_ps(y, result);
            z = _mm256_mul_ps(z, result);

            _mm256_storeu_ps(self.x.get_unchecked_mut(i), x);
            _mm256_storeu_ps(self.y.get_unchecked_mut(i), y);
            _mm256_storeu_ps(self.z.get_unchecked_mut(i), z);
        }
    }
}

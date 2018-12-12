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

    unsafe fn sse_add(&mut self, v: &Vectors3) {
        for i in (0..self.x.len()).step_by(4) {
            let ax = _mm_loadu_ps(self.x.get_unchecked(i));
            let ay = _mm_loadu_ps(self.y.get_unchecked(i));
            let az = _mm_loadu_ps(self.z.get_unchecked(i));

            let bx = _mm_loadu_ps(v.x.get_unchecked(i));
            let by = _mm_loadu_ps(v.y.get_unchecked(i));
            let bz = _mm_loadu_ps(v.z.get_unchecked(i));

            _mm_storeu_ps(self.x.get_unchecked_mut(i),_mm_add_ps(ax,bx));
            _mm_storeu_ps(self.y.get_unchecked_mut(i),_mm_add_ps(ay,by));
            _mm_storeu_ps(self.z.get_unchecked_mut(i),_mm_add_ps(az,bz));

        }
    }

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
        for i in (0..self.x.len()).step_by(4) {
            let ax = _mm_loadu_ps(self.x.get_unchecked(i));
            let ay = _mm_loadu_ps(self.y.get_unchecked(i));
            let az = _mm_loadu_ps(self.z.get_unchecked(i));

            let len =_mm_rsqrt_ps(_mm_add_ps(_mm_add_ps(_mm_mul_ps(ax,ax) ,_mm_mul_ps(ay,ay)), _mm_mul_ps(az,az)));
            
            _mm_storeu_ps(self.x.get_unchecked_mut(i),_mm_mul_ps(ax,len));
            _mm_storeu_ps(self.y.get_unchecked_mut(i),_mm_mul_ps(ay,len));
            _mm_storeu_ps(self.z.get_unchecked_mut(i),_mm_mul_ps(az,len));
        }
    }

    unsafe fn sse_clamp(&mut self, min: f32) {
         let true_result = _mm_set1_ps(min);
         let false_result = _mm_set1_ps(1.0);
         for i in (0..self.x.len()).step_by(4) {
            let ax = _mm_loadu_ps(self.x.get_unchecked(i));
            let ay = _mm_loadu_ps(self.y.get_unchecked(i));
            let az = _mm_loadu_ps(self.z.get_unchecked(i));

            let len =_mm_sqrt_ps(_mm_add_ps(_mm_add_ps(_mm_mul_ps(ax,ax) ,_mm_mul_ps(ay,ay)), _mm_mul_ps(az,az)));

            let mask = _mm_cmplt_ps(len,true_result);

            let result = _mm_or_ps(_mm_and_ps(mask,true_result), _mm_andnot_ps(mask,false_result));

            _mm_storeu_ps(self.x.get_unchecked_mut(i),_mm_mul_ps(_mm_div_ps(ax,len),result));
            _mm_storeu_ps(self.y.get_unchecked_mut(i),_mm_mul_ps(_mm_div_ps(ay,len),result));
            _mm_storeu_ps(self.z.get_unchecked_mut(i),_mm_mul_ps(_mm_div_ps(az,len),result));
        }      
    }

    #[target_feature(enable = "avx2")]
    unsafe fn avx_clamp(&mut self, min: f32) {
     let true_result = _mm256_set1_ps(min);
         let false_result = _mm256_set1_ps(1.0);
         for i in (0..self.x.len()).step_by(8) {
            let ax = _mm256_loadu_ps(self.x.get_unchecked(i));
            let ay = _mm256_loadu_ps(self.y.get_unchecked(i));
            let az = _mm256_loadu_ps(self.z.get_unchecked(i));

            let len =_mm256_sqrt_ps(_mm256_add_ps(_mm256_add_ps(_mm256_mul_ps(ax,ax) ,_mm256_mul_ps(ay,ay)), _mm256_mul_ps(az,az)));

            let mask = _mm256_cmp_ps(len,true_result,_CMP_LT_OS);

            let result = _mm256_blendv_ps(false_result,true_result,mask);
         
            _mm256_storeu_ps(self.x.get_unchecked_mut(i),_mm256_mul_ps(_mm256_div_ps(ax,len),result));
            _mm256_storeu_ps(self.y.get_unchecked_mut(i),_mm256_mul_ps(_mm256_div_ps(ay,len),result));
            _mm256_storeu_ps(self.z.get_unchecked_mut(i),_mm256_mul_ps(_mm256_div_ps(az,len),result));
        }      
    }
}

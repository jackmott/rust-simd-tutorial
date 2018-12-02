use std::arch::x86_64::*;


#[derive(Debug)]
pub struct Vector3 {
    pub x : f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub pos:Vector3,
    pub v: Vector3
}


pub trait Vector {
    fn add(&mut self, b: &Self);    
    fn mul(&mut self, scale: f32);
    fn norm(&mut self);
    fn simd_add(&mut self, b : &Self);
    fn simd_mul(&mut self, scale:f32);
    fn simd_norm(&mut self);
}


impl Vector for Vector3 {

    fn add(&mut self, b:&Vector3)  {        
        self.x = self.x + b.x;
        self.y = self.y + b.y;
        self.z = self.z + b.z;        
    }

    fn mul(&mut self, scale:f32) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }

    fn norm(&mut self) {
        let len = (self.x*self.x+self.y*self.y+self.z*self.z).sqrt();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }
    
    
    fn simd_add(&mut self, b:&Vector3)  {
        unsafe {            
            let a_simd = _mm_set_ps(self.x,self.y,self.z,0.0);
            let b_simd = _mm_set_ps(b.x,b.y,b.z,0.0);
            let result = _mm_add_ps(a_simd,b_simd);
            let result_arr = std::mem::transmute::<__m128,[f32;4]>(result);
            self.x = result_arr[3];
            self.y = result_arr[2];
            self.z = result_arr[1];            
        }
    }

     fn simd_mul(&mut self, scale:f32) {
         unsafe {
            let a_simd = _mm_set_ps(self.x,self.y,self.z,0.0);
            let b_simd = _mm_set1_ps(scale);
            let result = _mm_mul_ps(a_simd,b_simd);
            let result_arr = std::mem::transmute::<__m128,[f32;4]>(result);
            self.x = result_arr[3];
            self.y = result_arr[2];
            self.z = result_arr[1];
        }
    }

    fn simd_norm(&mut self) {
        //??
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_add() {
       let mut a = Vector3 { x:1.0, y:2.0, z:3.0 };
       let b = Vector3 { x:2.0, y:4.0, z:6.0 };
       a.simd_add(&b);
       assert_eq!(a.x,3.0);
       assert_eq!(a.y,6.0);
       assert_eq!(a.z,9.0);
    }
}
    




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
    pub v: Vector3,
    pub mass: f32,
    pub elasticity: f32,
    pub strength: f32
}


pub trait Vector {
    fn add(&mut self, b: &Self);        
    fn norm(&mut self);
    fn simd_add(&mut self, b : &Self);    
    fn simd_norm(&mut self);
}


impl Vector for Vector3 {

    fn add(&mut self, b:&Vector3)  {        
        self.x = self.x + b.x;
        self.y = self.y + b.y;
        self.z = self.z + b.z;        
    }

   

    fn norm(&mut self) {
        let len = (self.x*self.x+self.y*self.y+self.z*self.z).sqrt();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }
    
    
    fn simd_add(&mut self, b:&Vector3)  {
       
    }
     
    fn simd_norm(&mut self) {
       
    }
}







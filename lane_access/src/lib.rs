use std::arch::x86_64::*;

union SIMDArray {
    s: __m128,
    a: [f32; 4],
}

unsafe fn sum(n: Vec<f32>) -> f32 {
    
}

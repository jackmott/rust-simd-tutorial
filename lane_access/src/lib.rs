use std::arch::x86_64::*;

union SIMDArray {
    s: __m128,
    a: [f32; 4],
}

unsafe fn foo(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    let mut result = Vec::with_capacity(a.len());
    for i in (0..a.len()).step_by(4) {
        
        let a_union = SIMDArray {
            s: _mm_loadu_ps(a.get_unchecked(i)),
        };
        let b_union = SIMDArray {
            s: _mm_loadu_ps(a.get_unchecked(i)),
        };

        let mut sum_union = SIMDArray {
            s: _mm_add_ps(a_union.s, b_union.s),
        };

        sum_union.a[0] = sum_union.a[0].sin();
        sum_union.a[1] = sum_union.a[1].sin();
        sum_union.a[2] = sum_union.a[2].sin();
        sum_union.a[3] = sum_union.a[3].sin();

        _mm_storeu_ps(result.get_unchecked_mut(i), sum_union.s);
    }
    result
}

#[macro_use]
extern crate criterion;
extern crate rust_simd_tutorial;
extern crate rand;

use rand::Rng;
use rust_simd_tutorial::*;
use criterion::Criterion;

const TEST_SIZE : i32 = 100_000;

fn prepare_entities() -> Vec<Entity>  {
    // Setup, make some vectors
    let mut rng = rand::thread_rng();
    let mut result = Vec::new();
    for _ in 0 .. TEST_SIZE {     
        let pos = Vector3 { x:rng.gen::<f32>(),y:rng.gen::<f32>(),z:rng.gen::<f32>() };        
        let v = Vector3 { x:rng.gen::<f32>(),y:rng.gen::<f32>(),z:rng.gen::<f32>() };        
        result.push( Entity { name:"test".to_string(),pos,v,mass:1.0,elasticity:1.0,strength:1.0});
    }    
    result
}

fn move_entities(entities: &mut Vec<Entity>) {
    for e in entities.iter_mut() {
        e.pos.add(&e.v);
    }
}


fn move_entities_simd(entities: &mut Vec<Entity>) {
    for e in entities.iter_mut() {
       unsafe { e.pos.simd_add(&e.v) };
    }
}


fn benchmark(c: &mut Criterion) {
    let mut entities = prepare_entities();
    c.bench_function("normal move", move |b| b.iter(|| move_entities(&mut entities)));
    let mut entities = prepare_entities();
    c.bench_function("simd move", move |b| b.iter(|| move_entities_simd(&mut entities)));          
}



criterion_group!(benches,benchmark);
criterion_main!(benches);
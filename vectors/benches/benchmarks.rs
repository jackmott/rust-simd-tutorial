#[macro_use]
extern crate criterion;
extern crate rust_intrin;
extern crate rand;

use rand::Rng;
use rust_intrin::*;
use criterion::Criterion;

const TEST_SIZE : i32 = 100_000;

fn prepare_entities() -> Vec<Entity>  {
    // Setup, make some vectors
    let mut rng = rand::thread_rng();
    let mut result = Vec::new();
    for _ in 0 .. TEST_SIZE {     
        let pos = Vector3 { x:rng.gen::<f32>(),y:rng.gen::<f32>(),z:rng.gen::<f32>() };        
        let v = Vector3 { x:rng.gen::<f32>(),y:rng.gen::<f32>(),z:rng.gen::<f32>() };        
        result.push( Entity { name:"test".to_string(),pos,v});
    }    
    result
}

fn move_entities(entities: &mut Vec<Entity>) {
    for e in entities.iter_mut() {
        e.pos.add(&e.v);
    }
}

fn scale_entities(entities: &mut Vec<Entity>) {
    for e in entities.iter_mut() {
        e.pos.mul(2.0);
    }
}

fn move_entities_simd(entities: &mut Vec<Entity>) {
    for e in entities.iter_mut() {
       e.pos.simd_add(&e.v);
    }
}

fn scale_entities_simd(entities: &mut Vec<Entity>) {
    for e in entities.iter_mut() {
        e.pos.simd_mul(2.0);
    }
}



fn benchmark(c: &mut Criterion) {
    let mut entities = prepare_entities();
    c.bench_function("normal move", move |b| b.iter(|| move_entities(&mut entities)));
    let mut entities = prepare_entities();
    c.bench_function("simd move", move |b| b.iter(|| move_entities_simd(&mut entities)));

   
    let mut entities = prepare_entities();
    c.bench_function("normal scale", move |b| b.iter(|| scale_entities(&mut entities)));
    let mut entities = prepare_entities();
    c.bench_function("simd scale", move |b| b.iter(|| scale_entities_simd(&mut entities)));
       
}



criterion_group!(benches,benchmark);
criterion_main!(benches);
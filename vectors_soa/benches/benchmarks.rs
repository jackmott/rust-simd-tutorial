#[macro_use]
extern crate criterion;
extern crate rust_intrin;
extern crate rand;

use rand::Rng;
use rust_intrin::*;
use criterion::Criterion;


const TEST_SIZE : i32 = 100_000;


fn prepare_entities_soa() -> Entities {
    let mut rng = rand::thread_rng();    
    let pos = Vectors3 { x: Vec::new(), y: Vec::new(), z: Vec::new() };
    let v = Vectors3 { x: Vec::new(), y: Vec::new(), z: Vec::new() };
    let mut e = Entities { name: Vec::new(), pos, v};
    for _ in 0 .. TEST_SIZE {     
        e.name.push("test".to_string());
        e.pos.x.push(rng.gen::<f32>());
        e.pos.y.push(rng.gen::<f32>());
        e.pos.z.push(rng.gen::<f32>());
        e.v.x.push(rng.gen::<f32>());
        e.v.y.push(rng.gen::<f32>());
        e.v.z.push(rng.gen::<f32>());
    }    
    e
}

fn move_entities_soa(entities: &mut Entities)  {
    entities.pos.add(&entities.v);    
}

fn scale_entities_soa(entities: &mut Entities)  {
    entities.pos.mul(2.0);    
}

fn move_entities_soa_simd(entities: &mut Entities)  {
    entities.pos.simd_add(&entities.v);    
}

fn scale_entities_soa_simd(entities: &mut Entities)  {
    entities.pos.simd_mul(2.0);    
}



fn benchmark(c: &mut Criterion) {
  
    let mut entities = prepare_entities_soa();
    c.bench_function("soa move", move |b| b.iter(|| move_entities_soa(&mut entities)));

    let mut entities = prepare_entities_soa();
    c.bench_function("soa + simd move", move |b| b.iter(|| move_entities_soa_simd(&mut entities)));

    let mut entities = prepare_entities_soa();
    c.bench_function("soa scale", move |b| b.iter(|| scale_entities_soa(&mut entities)));

    let mut entities = prepare_entities_soa();
    c.bench_function("soa + simd scale", move |b| b.iter(|| scale_entities_soa_simd(&mut entities)));
    
}



criterion_group!(benches,benchmark);
criterion_main!(benches);
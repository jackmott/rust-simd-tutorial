#[macro_use]
extern crate criterion;
extern crate rand;
extern crate rust_intrin;

use criterion::Criterion;
use rand::Rng;
use rust_intrin::*;

const TEST_SIZE: i32 = 100_000;

fn prepare_entities_soa() -> Entities {
    let mut rng = rand::thread_rng();
    let pos = Vectors3 {
        x: Vec::new(),
        y: Vec::new(),
        z: Vec::new(),
    };
    let v = Vectors3 {
        x: Vec::new(),
        y: Vec::new(),
        z: Vec::new(),
    };
    let mut e = Entities {
        name: Vec::new(),
        pos,
        v,
        mass: Vec::new(),
        elasticity: Vec::new(),
        strength: Vec::new(),
    };
    for _ in 0..TEST_SIZE {
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

fn move_entities_soa(entities: &mut Entities) {
    entities.pos.add(&entities.v);
}

fn move_entities_soa_simd(entities: &mut Entities) {
    unsafe {
        entities.pos.simd_add(&entities.v);
    }
}

fn normalize_velocity_soa(entities: &mut Entities) {
    entities.pos.norm();
}

fn normalize_velocity_soa_sse(entities: &mut Entities) {
    unsafe {
        entities.pos.simd_norm();
    }
}

fn normalize_velocity_soa_avx(entities: &mut Entities) {
    unsafe {
        entities.pos.simd_norm_avx();
    }
}

fn normalize_velocity_soa_simd_avx(entities: &mut Entities) {}

fn benchmark(c: &mut Criterion) {
    let mut entities = prepare_entities_soa();
    c.bench_function("soa move", move |b| {
        b.iter(|| move_entities_soa(&mut entities))
    });
/*
    let mut entities = prepare_entities_soa();
    c.bench_function("soa + simd move", move |b| {
        b.iter(|| move_entities_soa_simd(&mut entities))
    });
    
    let mut entities = prepare_entities_soa();
    c.bench_function("soa norm", move |b| {
        b.iter(|| normalize_velocity_soa(&mut entities))
    });

    let mut entities = prepare_entities_soa();
    c.bench_function("soa + sse norm", move |b| {
        b.iter(|| normalize_velocity_soa_simd(&mut entities))
    });

    let mut entities = prepare_entities_soa();
    c.bench_function("soa + avx norm", move |b| {
        b.iter(|| normalize_velocity_soa_avx(&mut entities))
    });
    */
}

pub fn main() {
    let mut c = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(100))
        .measurement_time(std::time::Duration::from_millis(100))
        .sample_size(5)
        .without_plots();
    benchmark(&mut c);
}


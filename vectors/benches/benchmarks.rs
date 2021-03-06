extern crate criterion;
extern crate rand;
extern crate rust_simd_tutorial;

use criterion::Criterion;
use rand::Rng;
use rust_simd_tutorial::*;

const TEST_SIZE: i32 = 100_000;

fn prepare_entities() -> Vec<Entity> {
    // Setup, make some vectors
    let mut rng = rand::thread_rng();
    let mut result = Vec::new();
    for _ in 0..TEST_SIZE {
        let pos = Vector3 {
            x: rng.gen::<f32>(),
            y: rng.gen::<f32>(),
            z: rng.gen::<f32>(),
        };
        let v = Vector3 {
            x: rng.gen::<f32>(),
            y: rng.gen::<f32>(),
            z: rng.gen::<f32>(),
        };
        result.push(Entity {
            name: "test".to_string(),
            pos,
            v,
            mass: 1.0,
            elasticity: 1.0,
            strength: 1.0,
        });
    }
    result
}

fn move_entities(entities: &mut Vec<Entity>) {
    for e in entities.iter_mut() {
        e.pos.add(&e.v);
    }
}

fn sse_move_entities(entities: &mut Vec<Entity>) {
    for e in entities.iter_mut() {
        unsafe { e.pos.sse_add(&e.v) };
    }
}

fn benchmark(c: &mut Criterion) {
    let mut entities = prepare_entities();
    c.bench_function("scalar move", move |b| {
        b.iter(|| move_entities(&mut entities))
    });
    let mut entities = prepare_entities();
    c.bench_function("sse move", move |b| {
        b.iter(|| sse_move_entities(&mut entities))
    });
}

pub fn main() {
    let mut c = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(100))
        .measurement_time(std::time::Duration::from_millis(100))
        .sample_size(5)
        .without_plots();
    benchmark(&mut c);
}

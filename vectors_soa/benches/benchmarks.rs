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

fn move_entities(entities: &mut Entities) {
    entities.pos.add(&entities.v);
}

fn sse_move_entities(entities: &mut Entities) {
    unsafe {
        entities.pos.sse_add(&entities.v);
    }
}

fn norm(entities: &mut Entities) {
    entities.v.norm();
}

fn sse_norm(entities: &mut Entities) {
    unsafe {
        entities.v.sse_norm();
    }
}

fn clamp(entities: &mut Entities) {
    entities.v.clamp(0.5);
}

fn sse_clamp(entities: &mut Entities) {
    unsafe {
        entities.v.sse_clamp(0.5);
    }
}

fn avx_clamp(entities: &mut Entities) {
    unsafe {
        entities.v.avx_clamp(0.5);
    }
}

fn benchmark(c: &mut Criterion) {
    let mut entities = prepare_entities_soa();
    c.bench_function("scalar move", move |b| {
        b.iter(|| move_entities(&mut entities))
    });

/*
    let mut entities = prepare_entities_soa();
    c.bench_function("sse move", move |b| {
        b.iter(|| sse_move_entities(&mut entities))
    });
  
    let mut entities = prepare_entities_soa();
    c.bench_function("scalar norm", move |b| {
        b.iter(|| norm(&mut entities))
    });

    let mut entities = prepare_entities_soa();
    c.bench_function("sse norm", move |b| {
        b.iter(|| sse_norm(&mut entities))
    });

    let mut entities = prepare_entities_soa();
    c.bench_function("scalar clamp", move |b| {
        b.iter(|| clamp(&mut entities))
    });

    let mut entities = prepare_entities_soa();
    c.bench_function("sse clamp", move |b| {
        b.iter(|| sse_clamp(&mut entities))
    });

    let mut entities = prepare_entities_soa();
    c.bench_function("avx clamp", move |b| {
        b.iter(|| avx_clamp(&mut entities))
    });*/
  
}

pub fn main() {
    let mut c = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(100))
        .measurement_time(std::time::Duration::from_millis(100))
        .sample_size(5)
        .without_plots();
    benchmark(&mut c);
}


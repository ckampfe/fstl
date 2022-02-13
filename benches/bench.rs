#[macro_use]
extern crate criterion;

use criterion::Criterion;
use fstl::parse_stl;
use std::io::Read;

fn parse_stl_binary_big(c: &mut Criterion) {
    let mut root_vase = vec![];

    std::fs::File::open("./fixtures/Root_Vase.stl")
        .unwrap()
        .read_to_end(&mut root_vase)
        .unwrap();

    assert!(!root_vase.is_empty());

    let mut group = c.benchmark_group("big");

    group.sample_size(15);

    group.bench_function("parse_stl_root_vase_binary_big_unindexed", move |b| {
        b.iter(|| {
            let triangles = parse_stl(criterion::black_box(&root_vase)).unwrap();
            assert_eq!(triangles.len(), 596_736);
        })
    });

    group.finish();
}

criterion_group!(benches, parse_stl_binary_big,);
criterion_main!(benches);

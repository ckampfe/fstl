#[macro_use]
extern crate criterion;

use criterion::Criterion;
use fstl::parse_stl;
use std::io::Read;

fn parse_stl_binary_big(c: &mut Criterion) {
    let mut root_vase = criterion::black_box(vec![]);

    std::fs::File::open("./fixtures/Root_Vase.stl")
        .unwrap()
        .read_to_end(&mut root_vase)
        .unwrap();

    assert!(!root_vase.is_empty());

    let mut group = c.benchmark_group("big");

    group.sample_size(15);

    group.bench_function("parse_stl_root_vase_binary_big_unindexed", move |b| {
        b.iter(|| {
            let triangles = parse_stl(&mut root_vase).unwrap();
            assert_eq!(triangles.len(), 596_736);
        })
    });

    group.finish();
}

// fn parse_stl_binary(c: &mut Criterion) {
//     let moon_file = std::fs::File::open("./fixtures/MOON_PRISM_POWER_binary.stl").unwrap();
//     let mut moon = BufReader::new(&moon_file);
//
//     c.bench_function("parse_stl_moon_prism_power_binary", move |b| {
//         b.iter(|| parse_stl::<BufReader<&File>>(&mut moon))
//     });
// }
//
// fn parse_stl_ascii(c: &mut Criterion) {
//     let moon_file = std::fs::File::open("./fixtures/MOON_PRISM_POWER.stl").unwrap();
//     let mut moon = BufReader::new(&moon_file);
//
//     c.bench_function("parse_stl_moon_prism_power_ascii", move |b| {
//         b.iter(|| parse_stl::<BufReader<&File>>(&mut moon))
//     });
// }

criterion_group!(
    benches,
    parse_stl_binary_big,
    // parse_stl_binary,
    // parse_stl_ascii
);
criterion_main!(benches);

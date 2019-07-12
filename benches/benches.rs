#[macro_use]
extern crate criterion;
extern crate octo;

use criterion::black_box;
use criterion::Criterion;

use octo::octree::Octree;

fn bench(c: &mut Criterion) {
    c.bench_function(
        "new", 
        |b| b.iter(|| black_box(Octree::<u8>::new(16)))
    );

    let mut octree = Octree::<u8>::new(16).unwrap();
    c.bench_function(
        "insert",
        move |b| b.iter(|| {
            black_box(octree.insert([12, 6, 8], 255).unwrap());
        })
    );

    let mut octree = Octree::<u8>::new(16).unwrap();
    octree.insert([12, 6, 8], 255);
    c.bench_function(
        "at",
        move |b| b.iter(|| {
            black_box(octree.at([12, 6, 8]).unwrap());
        })
    );
}

criterion_group!(benches, bench);
criterion_main!(benches);
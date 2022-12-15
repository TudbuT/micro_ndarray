use criterion::{criterion_group, criterion_main, Criterion};

use micro_ndarray::Array;
use ndarray::Array2;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("micro_ndarry", |b| b.iter(micro_ndarry));
    c.bench_function("ndarry", |b| b.iter(ndarry));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn micro_ndarry() {
    let mut array = Array::new_with([5000, 4000], 0);
    array
        .iter_mut()
        .filter(|(loc, _)| loc[0] == 1)
        .for_each(|x| 
            *x.1 += x.0[1]
        );

    for y in 0..4 {
        for x in 0..5 {
            #[allow(clippy::no_effect)]
            array[[x, y]];
        }
    }
}

fn ndarry() {
    let mut array = Array2::<usize>::zeros((5000, 4000));
    array
        .indexed_iter_mut()
        .filter(|((x, _), _)| x == &1)
        .for_each(|((_, y), ele)| 
            *ele += y
        );

    for y in 0..4 {
        for x in 0..5 {
            #[allow(clippy::no_effect)]
            array[[x, y]];
        }
    }
}
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use micro_ndarray::Array as MicroArray;
use ndarray::{Array2, Array3, Array4, ArrayD, Dimension, IxDyn};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("micro_ndarry", |b| b.iter(micro_ndarry));
    c.bench_function("ndarry", |b| b.iter(ndarry));

    c.bench_function("micro_ndarry 3D", |b| b.iter(micro_ndarry_3));
    c.bench_function("ndarry 3D", |b| b.iter(ndarry_3));

    c.bench_function("micro_ndarry 4D", |b| b.iter(micro_ndarry_4));
    c.bench_function("ndarry 4D", |b| b.iter(ndarry_4));

    c.bench_function("micro_ndarry 7D", |b| b.iter(micro_ndarry_7));
    c.bench_function("ndarry 7D", |b| b.iter(ndarry_7));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn micro_ndarry() {
    let mut array = MicroArray::new_with([5000, 2000], 0);
    array
        .iter_mut()
        .filter(|([x, _], _)| x == &1)
        .for_each(|([_, y], ele)| *ele += y);

    for y in 0..4 {
        for x in 0..5 {
            #[allow(clippy::no_effect)]
            array[[x, y]];
        }
    }
}

fn ndarry() {
    let mut array = Array2::<usize>::zeros((5000, 2000));
    array
        .indexed_iter_mut()
        .filter(|((x, _), _)| x == &1)
        .for_each(|((_, y), ele)| *ele += y);

    for y in 0..4 {
        for x in 0..5 {
            #[allow(clippy::no_effect)]
            array[[x, y]];
        }
    }
}

fn micro_ndarry_3() {
    let mut array = MicroArray::new_with([200, 200, 250], 0);
    array
        .iter_mut()
        .filter(|([x, ..], _)| x == &1)
        .for_each(|([_, y, ..], ele)| *ele += y);

    for (dim, ele) in array.iter() {
        black_box((dim, ele));
    }
}

fn ndarry_3() {
    let mut array = Array3::<usize>::zeros((200, 200, 250));
    array
        .indexed_iter_mut()
        .filter(|((x, _, _), _)| x == &1)
        .for_each(|((_, y, _), ele)| *ele += y);

    for (dim, ele) in array.indexed_iter() {
        black_box((dim, ele));
    }
}

fn micro_ndarry_4() {
    let mut array = MicroArray::new_with([50, 50, 50, 80], 0);
    array
        .iter_mut()
        .filter(|([x, ..], _)| x == &1)
        .for_each(|([_, y, ..], ele)| *ele += y);

    for (dim, ele) in array.iter() {
        black_box((dim, ele));
    }
}

fn ndarry_4() {
    let mut array = Array4::<usize>::zeros((50, 50, 50, 80));
    array
        .indexed_iter_mut()
        .filter(|((x, ..), _)| x == &1)
        .for_each(|((_, y, ..), ele)| *ele += y);

    for (dim, ele) in array.indexed_iter() {
        black_box((dim, ele));
    }
}

fn micro_ndarry_7() {
    let mut array = MicroArray::new_with([10; 7], 0);
    array
        .iter_mut()
        .filter(|([x, ..], _)| x == &1)
        .for_each(|([_, y, ..], ele)| *ele += y);

    for (dim, ele) in array.iter() {
        black_box((dim, ele));
    }
}

fn ndarry_7() {
    let mut array = ArrayD::<usize>::zeros(IxDyn(&[10; 7]));

    for (dim, ele) in array.indexed_iter_mut() {
        let view = dim.as_array_view();
        let slice = view.as_slice().unwrap();

        if slice[0] == 1 {
            continue;
        }

        *ele += slice[1];
    }

    for (dim, ele) in array.indexed_iter() {
        black_box((dim, ele));
    }
}

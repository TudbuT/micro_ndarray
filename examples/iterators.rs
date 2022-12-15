use micro_ndarray::Array;

fn main() {
    let mut array = Array::new_with([5, 4], 0);
    array
        .iterable_mut()
        .into_iter()
        .filter(|(loc, _)| loc[0] == 1)
        .for_each(|x| {
            println!("{x:?}");
            *x.1 += x.0[1];
        });
    for y in 0..4 {
        for x in 0..5 {
            print!("{}", array[[x, y]]);
        }
        println!();
    }
    assert_eq!(
        array
            .iterable()
            .into_iter()
            .map(|x| *x.1)
            .collect::<Vec<_>>(),
        vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0]
    )
}

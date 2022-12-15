use std::time::SystemTime;

use micro_ndarray::Array;

fn main() {
    let t = SystemTime::now();
    let mut array = Array::new_with([5000, 4000], 0);
    array
        .iter_mut()
        .filter(|(loc, _)| loc[0] == 1)
        .for_each(|x| {
            *x.1 += x.0[1];
        });
    for y in 0..4 {
        for x in 0..5 {
            print!("{}", array[[x, y]]);
        }
        println!();
    }
    println!(
        "Took {}ms.",
        SystemTime::now().duration_since(t).unwrap().as_millis()
    );
}

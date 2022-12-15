use micro_ndarray::Array;

fn main() {
    let mut array = Array::new_with([2, 2], 0);
    array[[0, 0]] = 1;
    array[[1, 0]] = 2;
    array[[0, 1]] = 3;
    array[[1, 1]] = 4;
    for y in 0..2 {
        for x in 0..2 {
            print!("{}", array[[x, y]]);
        }
        println!();
    }
}

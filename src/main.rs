extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let arr = (0..)
        .map(|_x| rng.gen_range(0, 10))
        .map(f64::from)
        .take(10)
        .collect::<Vec<_>>();

    let size = arr.len();
    let arr = arr.chunks(2).cycle().take(2 * size);

    println!("{:?}", arr.collect::<Vec<_>>());
}

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
    let res = arr.chunks(2).cycle().take(size);

    let res = res.enumerate().map(|(i, x)| {
        // println!("{}: {}", i, (-1f64).powi((i / (size / 2)) as i32));
        (x[0] + x[1] * (-1f64).powi((i / (size / 2)) as i32)) / 2.
    });

    println!("{:?}", arr);
    println!("{:?}", res.collect::<Vec<_>>());
}

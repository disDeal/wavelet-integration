extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let arr = (0..)
        .map(|_x| rng.gen_range(0, 10))
        .map(f64::from)
        .take(10)
        .collect::<Vec<_>>();

    let wave = |res: &Vec<f64>| {
        let size = res.len();
        arr.chunks(2)
            .cycle()
            .take(size)
            .map(|x| (x[0], x[1]))
            .enumerate()
            .map(|(i, x)| {
                let frac = (-1f64).powi((i / (size / 2)) as i32);
                (x.0 + x.1 * frac) / 2.
            })
            .collect::<Vec<_>>()
    };

    // for size in (0..size / 2 - 1).map(|n| size / (2 * n)).rev() {
    // }
    let res = wave(&arr);

    println!("{:?}", arr);
    println!("{:?}", res);
}

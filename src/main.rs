extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let arr = (0..).map(|_x| rng.gen_range(0, 10)).map(f64::from);

    println!("{:?}", arr.take(10).collect::<Vec<_>>());
}

extern crate rand;
// use rand::Rng;

extern crate plotlib;
use plotlib::page::Page;
use plotlib::repr::Line;
use plotlib::view::ContinuousView;

fn main() {
    // let mut rng = rand::thread_rng();
    // let arr = (0..)
    //     .map(|_x| rng.gen_range(0, 10))
    //     .map(f64::from)
    //     .take(10)
    //     .collect::<Vec<_>>();
    let arr = vec![1., 2., 3., 4., 5., 6., 7., 8., 9.];
    let size = arr.len();

    println!("{:?}", &arr);

    let mut res = arr.clone();
    for step in 1..size / 2 {
        res = wave(&res, step);
        println!("{:?}", &res);
    }

    let data = (0..size).map(|x| x as f64).zip(arr).collect::<Vec<_>>();
    let l1: Line = Line::new(data).style(plotlib::style::LineStyle::new().colour("#113355"));

    let data = (0..size).map(|x| x as f64).zip(res).collect::<Vec<_>>();
    let l2: Line = Line::new(data).style(plotlib::style::LineStyle::new().colour("#DD3355"));

    let view = ContinuousView::new()
        .add(l1)
        .add(l2)
        .x_label("Йекс")
        .y_label("Уигрек");

    Page::single(&view).save("wave.svg").unwrap();
}

fn wave(res: &[f64], step: usize) -> Vec<f64> {
    let size = res.len();
    let factor = size / (2 * step);
    res.chunks_exact(2)
        .cycle()
        .take(size)
        .enumerate()
        .collect::<Vec<_>>()
        .chunks_exact(factor)
        .flat_map(|arr| {
            // println!("{:?}", arr);
            arr.iter().map(|(i, x)| {
                let frac = (-1f64).powi((i / factor) as i32);
                // println!("{}: \t{}", i, frac);
                (x[0] + x[1] * frac)
            })
        })
        .collect::<Vec<_>>()
}

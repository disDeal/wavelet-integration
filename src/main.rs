extern crate rand;
use rand::Rng;

extern crate plotlib;
use plotlib::page::Page;
use plotlib::repr::{Line, Scatter};
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

fn main() {
    let mut rng = rand::thread_rng();
    let arr = (0..)
        .map(|_x| rng.gen_range(0, 10))
        .map(f64::from)
        .take(10)
        .collect::<Vec<_>>();
    let size = arr.len();

    let wave = |res: &Vec<f64>, step| {
        let size = res.len();
        arr.chunks(2)
            .cycle()
            .take(size)
            .map(|x| (x[0], x[1]))
            .enumerate()
            .map(|(i, x)| {
                let frac = (-1f64).powi((i / (step / 2)) as i32);
                (x.0 + x.1 * frac) / 2.
            })
            .collect::<Vec<_>>()
    };

    let mut res = arr.clone();
    for step in (1..size / 2).map(|n| size / (2 * n)).rev() {}
    res = wave(&res, size / 2);

    println!("{:?}", &arr);
    println!("{:?}", &res);

    let size = arr.len();
    let data = (0..size).map(|x| x as f64).zip(arr).collect::<Vec<_>>();
    let s2: Scatter = Scatter::from_slice(data.as_slice()).style(
        PointStyle::new()
            .marker(PointMarker::Square)
            .colour("#DD3355"),
    );

    let data = (0..size).map(|x| x as f64).zip(res).collect::<Vec<_>>();
    let l2: Line = Line::new(data).style(plotlib::style::LineStyle::new().colour("#DD3355"));

    let view = ContinuousView::new()
        .add(s2)
        .add(l2)
        .x_label("Йекс")
        .y_label("Уигрек");

    Page::single(&view).save("wave.svg").unwrap();
}

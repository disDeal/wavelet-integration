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
        .take(8)
        .collect::<Vec<_>>();
    // let arr = vec![1., 2., 3., 4., 5., 6., 7., 8.];
    let size = arr.len();


    println!("Input:");
    println!("{:?}", arr);

    println!("Straight transformation:");
    let res = wavelet(&arr);

    println!("Inverse  transformation:");
    let rev = wavelet_rev(&res);

    let data = (0..size).map(|x| x as f64).zip(arr).collect::<Vec<_>>();
    let l1: Line = Line::new(data).style(plotlib::style::LineStyle::new().colour("#113355"));

    let data = (0..size).map(|x| x as f64).zip(res).collect::<Vec<_>>();
    let l2: Line = Line::new(data).style(plotlib::style::LineStyle::new().colour("#DD3355"));

    let data = (0..size).map(|x| x as f64).zip(rev).collect::<Vec<_>>();
    let s1: Scatter = Scatter::from_slice(&data.as_slice()).style(
        PointStyle::new()
            .marker(PointMarker::Square)
            .colour("#DD3355"),
    );
    let view = ContinuousView::new()
        .add(l1)
        .add(l2)
        .add(s1)
        .x_label("Йекс")
        .y_label("Уигрек");

    Page::single(&view).save("wave2.svg").unwrap();
}

fn wavelet(arr: &[f64]) -> Vec<f64> {
    let size = arr.len();
    let mut acc = vec![0.; size];
    let mut res = vec![0.; size];
    acc.clone_from_slice(arr);

    for j in 1..size / 2 {
        let off = size / (2 * j);
        for i in 0..size / 2 {
            let factor = i / off;
            res[i + off * factor] = (acc[2 * i] + acc[2 * i + 1]) / 2.;
            res[i + off * factor + off] = (acc[2 * i + 1] - acc[2 * i]) / 2.;
        }
        println!("{:?}", res);
        acc.clone_from_slice(&res);
    }
    res.to_vec()
}

fn wavelet_rev(arr: &[f64]) -> Vec<f64> {
    let size = arr.len();
    let mut acc = vec![0.; size];
    let mut res = vec![0.; size];
    acc.clone_from_slice(arr);

    for j in 1..size / 2 {
        let off = size / (2 * j);
        for i in 0..size / 2 {
            let factor = i / off;
            res[i + off * factor] = acc[2 * i] - acc[2 * i + 1];
            res[i + off * factor + off] = acc[2 * i] + acc[2 * i + 1];
        }
        println!("{:?}", res);
        acc.clone_from_slice(&res);
    }
    res.to_vec()
}

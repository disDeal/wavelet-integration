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
    let arr = vec![1., 2., 3., 4., 5., 6., 7., 8., 9f64];
    let size = arr.len();

    let res = arr.clone();
    let factor = |n: usize| size / (2 * n);
    let mean = |x: &[f64]| x[0] + x[1];
    let div = |x: &[f64]| x[0] - x[1];

    println!("Pairs:");
    let pairs = res.chunks_exact(2).collect::<Vec<_>>();
    println!("{:?}", pairs);

    let group = |n: usize| pairs.chunks_exact(factor(n));

    println!("Groups:");
    println!("{:?}", group(1).collect::<Vec<_>>());
    println!("{:?}", group(2).collect::<Vec<_>>());
    println!("{:?}", group(3).collect::<Vec<_>>());

    let wave = |n| {
        let m = group(n).map(|arr| arr.iter().map(|x| mean(x)).collect::<Vec<_>>());
        let d = group(n).map(|arr| arr.iter().map(|x| div(x)).collect::<Vec<_>>());

        m.zip(d)
    };

    println!("Zips:");
    println!("{:?}", wave(1).collect::<Vec<_>>());
    println!("{:?}", wave(2).collect::<Vec<_>>());
    println!("{:?}", wave(3).collect::<Vec<_>>());

    let w3 = wave(2).collect::<Vec<_>>();

    let t1 = w3
        .into_iter()
        .map(|tuple| [tuple.0, tuple.1])
        .collect::<Vec<_>>();

    println!("Flattens");
    println!("{:?}", t1);

    let f1 = t1.iter().flatten().flatten().collect::<Vec<_>>();
    println!("{:?}", &f1);

    // let f2 = f1.iter().flatten();
    // println!("{:?}", f2);

    println!("Arrays:");
    println!("{:?}", arr);
    println!("{:?}", res);

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

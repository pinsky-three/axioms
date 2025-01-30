use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new("plot_example.svg", (1800, 1800)).into_drawing_area();

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .build_cartesian_2d(-1f32..1f32, -1f32..1f32)?;

    let f = |x| f32::abs(f32::sin(x * 10.0) * f32::cos(x * 20.0));

    chart.draw_series(LineSeries::new(
        (-1000..=1000).map(|x| x as f32 / 1000.0).map(|x| (x, f(x))),
        &GREEN,
    ))?;

    root.present()?;

    Ok(())
}

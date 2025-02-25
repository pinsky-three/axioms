use num::{complex::Complex64, Complex};
use plotters::{
    chart::ChartBuilder,
    coord::ranged1d::AsRangedCoord,
    prelude::{IntoDrawingArea, SVGBackend},
    series::LineSeries,
    style::full_palette::RED_100,
};
use std::iter;

pub fn generate_grid(
    start: Complex64,
    end: Complex64,
    step: f64,
) -> impl Iterator<Item = Complex64> {
    let re_range = start.re..end.re;
    let im_range = start.im..end.im;

    let re_values = iter::successors(Some(re_range.start), move |x| {
        let step = step;
        let re_end = re_range.end;

        if *x + step < re_end {
            Some(*x + step)
        } else {
            None
        }
    });

    let im_values = iter::successors(Some(im_range.start), move |x| {
        let step = step;
        let im_end = im_range.end;

        if *x + step < im_end {
            Some(*x + step)
        } else {
            None
        }
    });

    re_values.flat_map(move |re| im_values.clone().map(move |im| Complex64::new(re, im)))
}

pub fn generate_graph<'a, X, Y>(
    x_spec: X,
    y_spec: Y,
    start_grid: Complex<f64>,
    end_grid: Complex<f64>,
    step_grid: f64,
    transformation: impl FnMut(Complex64) -> Complex64,
) -> Result<Vec<u8>, Box<dyn std::error::Error>>
where
    X: AsRangedCoord + 'static,
    Y: AsRangedCoord + 'static,
    for<'b> &'b plotters::element::DynElement<'static, plotters::prelude::SVGBackend<'a>, (f32, f32)>:
        plotters::element::PointCollection<
            'b,
            (
                <X as plotters::coord::ranged1d::AsRangedCoord>::Value,
                <Y as plotters::coord::ranged1d::AsRangedCoord>::Value,
            ),
        >,
{
    let root_path = "plot_example.svg";

    let root = SVGBackend::new(root_path, (1800, 1800)).into_drawing_area();

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .build_cartesian_2d(x_spec, y_spec)?;

    let grid1 = generate_grid(start_grid, end_grid, step_grid);

    let f = transformation;

    let transformed_grid = grid1.map(f);
    let points = transformed_grid.map(|z| (z.re as f32, z.im as f32));

    chart.draw_series(LineSeries::new(points, &RED_100))?;

    root.present()?;
    let svg_data = std::fs::read(root_path)?;

    std::fs::write("assets/plot_example.svg", svg_data.clone())?;

    Ok(svg_data)
}

pub fn generate_gcode(svg_data: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    let program = svg2gcode::svg2program(
        &roxmltree::Document::parse(String::from_utf8(svg_data).unwrap().as_str()).unwrap(),
        &svg2gcode::ConversionConfig {
            dpi: 100.0,
            feedrate: 2000.0,
            origin: [Some(48.0), Some(36.0)],
            tolerance: 0.004,
        },
        svg2gcode::ConversionOptions {
            dimensions: [
                Some(svgtypes::Length {
                    number: 199.0,
                    unit: svgtypes::LengthUnit::Mm,
                }),
                Some(svgtypes::Length {
                    number: 199.0,
                    unit: svgtypes::LengthUnit::Mm,
                }),
            ],
        },
        svg2gcode::Machine::new(
            svg2gcode::SupportedFunctionality {
                ..Default::default()
            },
            Some(g_code::parse::snippet_parser("G1 Z0").unwrap()),
            Some(g_code::parse::snippet_parser("G1 Z3").unwrap()),
            Some(g_code::parse::snippet_parser("G28 G1 Z10 G1 X43 Y31 Z1 G1 Z0").unwrap()),
            Some(g_code::parse::snippet_parser("G1 Z30").unwrap()),
        ),
    );

    let mut writable_gcode_file = std::fs::File::create("plot_example.gcode")?;

    let mut gcode_string = String::new();

    g_code::emit::format_gcode_fmt(
        &program,
        g_code::emit::FormatOptions {
            ..Default::default()
        },
        &mut gcode_string,
    )
    .unwrap();

    use std::io::Write;
    writable_gcode_file.write_all(gcode_string.as_bytes())?;

    Ok(gcode_string)
}

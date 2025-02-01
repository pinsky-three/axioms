use g_code::emit::{format_gcode_fmt, FormatOptions};
use num::complex::Complex64;
use plotters::prelude::*;
use std::iter;

fn calculate_points() -> impl Iterator<Item = (f32, f32)> {
    let f = |x: Complex64| (x.sqrt() + 1.).tan();

    let start_range = Complex64::new(-1.0, -1.0);
    let end_range = Complex64::new(1.0, 1.0);

    let step = 0.08;

    let re_range = start_range.re..end_range.re;
    let im_range = start_range.im..end_range.im;

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

    let complex_plane = re_values.flat_map(move |re| {
        im_values.clone().map(move |im| {
            Complex64::new(re, im)
            // (z.re as f32, z.im as f32, f(z).re as f32)
        })
    });

    complex_plane.map(f).map(|z| (z.re as f32, z.im as f32))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_path = "plot_example.svg";

    let root = SVGBackend::new(root_path, (1800, 1800)).into_drawing_area();

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .build_cartesian_2d(-2f32..2f32, -2f32..2f32)?;

    // let f = |x| f32::abs(f32::sin(x * 10.0) * f32::cos(x * 20.0));

    let points = calculate_points();

    chart.draw_series(LineSeries::new(points, &GREEN))?;

    root.present()?;

    let svg_data = std::fs::read(root_path)?;

    let program = svg2gcode::svg2program(
        &roxmltree::Document::parse(String::from_utf8(svg_data).unwrap().as_str()).unwrap(),
        &svg2gcode::ConversionConfig {
            dpi: 100.0,
            feedrate: 300.0,
            origin: [Some(52.0), Some(36.0)],
            tolerance: 0.005,
        },
        svg2gcode::ConversionOptions {
            dimensions: [
                Some(svgtypes::Length {
                    number: 100.0,
                    unit: svgtypes::LengthUnit::Mm,
                }),
                Some(svgtypes::Length {
                    number: 100.0,
                    unit: svgtypes::LengthUnit::Mm,
                }),
            ],
        },
        svg2gcode::Machine::new(
            svg2gcode::SupportedFunctionality {
                ..Default::default()
            },
            None,
            None,
            Some(g_code::parse::snippet_parser("G28 G1 Z10 G1 X42 Y30 Z1 G1 Z0").unwrap()),
            Some(g_code::parse::snippet_parser("G1 Z30").unwrap()),
        ),
    );

    let mut writable_gcode_file = std::fs::File::create("plot_example.gcode")?;

    let mut gcode_string = String::new();

    format_gcode_fmt(
        &program,
        FormatOptions {
            ..Default::default()
        },
        &mut gcode_string,
    )
    .unwrap();

    use std::io::Write;
    writable_gcode_file.write_all(gcode_string.as_bytes())?;

    // let program_data = program
    //     .iter()
    //     .map(|t| match t {
    //         Token::Field(_) => "Field",
    //         Token::Comment(_, _) => "Comment",
    //         Token::Command(_) => "Command",
    //     })
    //     .collect::<Vec<String>>()
    //     .join(" ");

    // println!("{}", program_data);

    // std::fs::write("plot_example.gcode", program_data)?;

    Ok(())
}

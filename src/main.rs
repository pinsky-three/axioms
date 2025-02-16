use axioms::Transformations;
use g_code::emit::{format_gcode_fmt, FormatOptions};
use num::complex::Complex64;
use plotters::prelude::*;
use std::iter;

fn generate_grid(start: Complex64, end: Complex64) -> impl Iterator<Item = Complex64> {
    let step = 0.15;

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

// fn calculate_points(
//     complex_plane: impl Iterator<Item = Complex64>,
// ) -> impl Iterator<Item = (f32, f32)> {

// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_path = "plot_example.svg";

    let root = SVGBackend::new(root_path, (1800, 1800)).into_drawing_area();

    // root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .build_cartesian_2d(-10f32..10f32, -10f32..10f32)?;

    // chart
    //     .configure_mesh()
    //     // .disable_x_axis()
    //     // .disable_y_axis()
    //     .disable_x_mesh()
    //     .disable_y_mesh()
    //     // .set_all_tick_mark_size(1000000)
    //     .draw()?;

    let square_sample_length = 1.4;

    let start_range = Complex64::new(-square_sample_length, -square_sample_length);
    let end_range = Complex64::new(square_sample_length, square_sample_length);

    // let original_grid = generate_grid(start_range, end_range);
    let grid1 = generate_grid(start_range, end_range);

    // let complex_plane = generate_grid(start_range, end_range);

    let f = |z: Complex64| {
        // Transformations::inverse_transformation(z)
        Transformations::parabolic_transformation(z)
    };

    let transformed_grid = grid1.map(f);

    // let points = original_grid.zip(transformed_grid).map(|(z1, z2)| z1 - z2);
    let points = transformed_grid.map(|z| (z.re as f32, z.im as f32));

    // let original_points = grid1.map(|z| (z.re as f32, z.im as f32));
    // let points = complex_plane.map(f).map(|z| (z.re as f32, z.im as f32));

    // chart.draw_series(LineSeries::new(
    //     grid1
    //         .map(|z| {
    //             let z1 = z * 2.0;

    //             (z1.re as f32, z1.im as f32)
    //         })
    //         .collect::<Vec<_>>(),
    //     &GREEN,
    // ))?;

    // root.titled("Mobius Transformation", ("sans-serif", 60))?;

    chart.draw_series(LineSeries::new(points, &RED))?;

    // chart.draw_series(PointSeries::of_element(points, 1, &GREEN, &|c, s, st| {
    //     EmptyElement::at(c) + Circle::new((0, 0), s, st.filled())
    // }))?;

    // chart.draw_series(PointSeries::of_element(points, 2, &BLACK, &|c, s, st| {
    //     EmptyElement::at(c) + Circle::new((0, 0), s, st.filled())
    // }))?;

    root.present()?;
    let svg_data = std::fs::read(root_path)?;

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
                    number: 200.0,
                    unit: svgtypes::LengthUnit::Mm,
                }),
                Some(svgtypes::Length {
                    number: 200.0,
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

use g_code::emit::{format_gcode_fmt, FormatOptions};
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_path = "plot_example.svg";

    let root = SVGBackend::new(root_path, (1800, 1800)).into_drawing_area();

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .build_cartesian_2d(-1f32..1f32, -1f32..1f32)?;

    let f = |x| f32::abs(f32::sin(x * 10.0) * f32::cos(x * 20.0));

    chart.draw_series(LineSeries::new(
        (-1000..=1000).map(|x| x as f32 / 1000.0).map(|x| (x, f(x))),
        &GREEN,
    ))?;

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

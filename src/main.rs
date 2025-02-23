use axioms::{
    common,
    generators::{generate_gcode, generate_graph},
};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_svg::prelude::*;
// use evalexpr::{ContextWithMutableVariables, DefaultNumericTypes, HashMapContext};
// use num::complex::Complex64;
// use g_code::emit::{format_gcode_fmt, FormatOptions};
// use num::complex::Complex64;
// use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "origin_check".to_string(),
                resolution: (1200., 1200.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((
            common::CommonPlugin,
            bevy_svg::prelude::SvgPlugin,
            EguiPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example_system)
        .run();

    Ok(())
}

fn setup(mut commands: Commands) {
    // asset_server: Res<AssetServer>
    commands.spawn(Camera2d);

    // commands.spawn((Svg2d(svg.clone()), Origin::Center));
    // commands.spawn((Svg2d(svg), Origin::TopLeft, common::DontChange));
}

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/minimal_complex_math.pest"]
struct MinimalComplexMathParser;

#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        let mut value = "world".to_string();

        ui.label("world");
        ui.text_edit_singleline(&mut value);
        if ui.button("Click me").clicked() {
            println!("Button clicked: {}", value);

            let func: &str = "-0.3z^2 + 1.2e^.4*pi*i";

            let res = MinimalComplexMathParser::parse(Rule::expression, func).unwrap();

            println!("{:?}", res);

            // let mut context = HashMapContext::<DefaultNumericTypes>::new();

            // context
            //     .set_value("z".to_string(), evalexpr::Value::from_float(0.0))
            //     .unwrap();

            // context
            //     .set_value("i".to_string(), evalexpr::Value::from("sqrt(-1)"))
            //     .unwrap();

            // let res = evalexpr::eval_with_context(func, &context).unwrap();
            // println!("res: {}", res);

            let svg_data = generate_graph().unwrap();
            let _g_code = generate_gcode(svg_data).unwrap();

            let svg = asset_server.load("plot_example.svg");
            commands.spawn((Svg2d(svg), Origin::Center));
        }
    });
}

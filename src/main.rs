use axioms::{
    common,
    generators::{generate_gcode, generate_graph},
    grammar::{MinimalComplexMathParser, Rule},
};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_svg::prelude::*;
use pest::{iterators::Pair, Parser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    commands.spawn(Camera2d);
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

            let svg_data = generate_graph().unwrap();
            let _g_code = generate_gcode(svg_data).unwrap();

            let svg = asset_server.load("plot_example.svg");

            _calculate_expr();

            commands.spawn((Svg2d(svg), Origin::Center));
        }
    });
}

fn walk_pairs(pair: Pair<Rule>, indent: usize) {
    // Create an indent string for pretty printing.
    let indent_str = "  ".repeat(indent);
    println!(
        "{}Rule: {:?} | Text: {:?}",
        indent_str,
        pair.as_rule(),
        pair.as_str()
    );

    // Recursively process all inner pairs.
    for inner_pair in pair.into_inner() {
        walk_pairs(inner_pair, indent + 1);
    }
}

fn _calculate_expr() {
    let func: &str = "-0.3z^2 + 2e^(.4*pi*i)";

    // Parse the input using the top-level rule 'expression'
    let parse_result = MinimalComplexMathParser::parse(Rule::expression, func)
        .unwrap_or_else(|e| panic!("Parsing error: {}", e));

    // Walk over each pair in the parse result.
    for pair in parse_result {
        walk_pairs(pair, 0);
    }
}

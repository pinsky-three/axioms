use std::collections::HashMap;

use axioms::{
    common,
    generators::{generate_gcode, generate_graph},
    grammar::ComplexMath,
};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_svg::prelude::*;
use num::{complex::Complex64, Zero};
// use pest::{iterators::Pair, Parser};

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
        let mut value = "-0.3z^2 + 2e^(.4*pi*i)".to_string();

        ui.label("expression");
        ui.text_edit_singleline(&mut value);

        if ui.button("Click me").clicked() {
            println!("Button clicked: {}", value);

            let svg_data = generate_graph().unwrap();
            let _g_code = generate_gcode(svg_data).unwrap();

            let svg = asset_server.load("plot_example.svg");

            // calculate_expr(&value);
            let mut ctx = HashMap::new();

            ctx.insert("z", Complex64::zero());

            ctx.insert("pi", Complex64::new(std::f64::consts::PI, 0.0));
            ctx.insert("i", Complex64::i());

            ComplexMath::calculate_expr(&ctx, &value).unwrap();

            commands.spawn((Svg2d(svg), Origin::Center));
        }
    });
}

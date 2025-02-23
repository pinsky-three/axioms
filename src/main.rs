use axioms::{
    common,
    generators::{generate_gcode, generate_graph},
    grammar::{MinimalComplexMathParser, Rule},
};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_svg::prelude::*;
use pest::Parser;

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

            let func: &str = "-0.3z^2 + 1.2e^.4*pi*i";

            let res = MinimalComplexMathParser::parse(Rule::expression, func).unwrap();

            println!("{:?}", res);

            let svg_data = generate_graph().unwrap();
            let _g_code = generate_gcode(svg_data).unwrap();

            let svg = asset_server.load("plot_example.svg");
            commands.spawn((Svg2d(svg), Origin::Center));
        }
    });
}

use axioms::{
    common,
    generators::{generate_gcode, generate_graph},
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_svg::prelude::*;
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
            commands.spawn((Svg2d(svg), Origin::Center));
        }
    });
}

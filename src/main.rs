use std::collections::HashMap;

use axioms::{
    common,
    generators::{generate_gcode, generate_graph},
    grammar::ComplexMath,
};

use bevy::{color::palettes::css::WHITE, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_svg::prelude::*;
use num::complex::Complex64;

#[derive(Default, Resource)]
struct ToolKitState {
    expression: String,
    translation_x: f32,
    translation_y: f32,
    entities: Vec<Entity>,
    grid_step: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "origin_check".to_string(),
                resolution: (1800., 1200.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((
            common::CommonPlugin,
            bevy_svg::prelude::SvgPlugin,
            EguiPlugin,
        ))
        .insert_resource(ToolKitState {
            expression: "2.2e*(-i*.2*z) + .4z^2".to_string(),
            translation_x: -830.0,
            translation_y: 1000.0,
            entities: Vec::new(),
            grid_step: 0.12,
        })
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
    mut state: ResMut<ToolKitState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    egui::Window::new("Tool Kit").show(contexts.ctx_mut(), |ui| {
        ui.label("expression");
        ui.text_edit_singleline(&mut state.expression);

        ui.add(egui::Slider::new(&mut state.translation_x, -1000.0..=1000.0).text("translation_x"));
        ui.add(egui::Slider::new(&mut state.translation_y, -1000.0..=1000.0).text("translation_y"));
        ui.add(egui::Slider::new(&mut state.grid_step, 0.01..=1.0).text("grid_step"));

        if ui.button("Compute").clicked() {
            println!("Button clicked: {}", state.expression);

            let value = state.expression.as_str();
            let mut ctx = HashMap::new();

            ctx.insert("pi", Complex64::new(std::f64::consts::PI, 0.0));
            ctx.insert("e", Complex64::new(std::f64::consts::E, 0.0));
            ctx.insert("i", Complex64::i());

            let svg_data = generate_graph(
                -2.0f32..2.0,
                -2.0f32..2.0,
                Complex64::new(-1.0, -1.0),
                Complex64::new(1.0, 1.0),
                state.grid_step,
                |z| {
                    ctx.insert("z", z);
                    ComplexMath::calculate_expr(&ctx, value).unwrap()
                },
            )
            .unwrap();

            let _g_code = generate_gcode(svg_data.clone()).unwrap();

            let svg = Svg::from_bytes(&svg_data, "plot_example.svg", Option::<&str>::None).unwrap();

            // println!("svg.size: {:?}", svg.size);
            // println!("svg.view_box: {:?}", svg.view_box);
            // println!("svg.mesh: {:?}", svg.mesh);

            // let svg = svgs.add(svg);
            // println!("svg: {:?}", svg);

            // let id = AssetId::Uuid {
            //     uuid: Uuid::new_v4(),
            // };

            // svgs.insert(id, svg);

            // let svg = svgs.get_strong_handle(id).unwrap();

            // let svg = asset_server.add(svg);

            // let a = asset_server.load(svg.path().unwrap().to_owned());

            // let svg = asset_server.load("plot_example.svg");
            // println!("svg: {:?}", svg);

            // commands.spawn((Svg2d(a), Origin::Center));

            let mesh = svg.tessellate();

            // println!("mesh: {:?}", mesh);

            let ent = commands.spawn((
                Mesh2d(meshes.add(mesh)),
                MeshMaterial2d(materials.add(Color::from(WHITE))),
                Transform::default().with_translation(Vec3::from_array([
                    state.translation_x,
                    state.translation_y,
                    0.0,
                ])),
            ));

            // save ent.id() to be able to remove it later
            state.entities.push(ent.id());
        }

        if ui.button("Clear").clicked() {
            for ent in state.entities.iter() {
                commands.entity(*ent).despawn();
            }

            state.entities.clear();
        }
    });
}

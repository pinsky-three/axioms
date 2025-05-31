// use std::collections::HashMap;

use engine::{
    camera,
    generators::{generate_gcode, generate_graph},
    grammar::{ComplexMath, ComplexMathContext},
};

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32},
    EguiContexts, EguiPlugin,
};
use bevy_svg::prelude::*;
use num::complex::Complex64;

#[derive(Default, Resource)]
struct ToolKitState {
    expression: String,
    abs_spec: f32,
    abs_grid: f64,
    entities: Vec<Entity>,
    grid_step: f64,
    path_color: Color32,
    background_color: Color32,
    loaded_svg_pah: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Axioms".to_string(),
                resolution: (1800., 1200.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((
            camera::CommonPlugin,
            bevy_svg::prelude::SvgPlugin,
            EguiPlugin,
        ))
        .insert_resource(ToolKitState {
            expression: "2.2e*(-i*.2*z) + .4z^2".to_string(),
            abs_spec: 2.0,
            abs_grid: 1.0,
            entities: Vec::new(),
            grid_step: 0.04,
            path_color: Color32::from_gray(128),
            background_color: Color32::from_gray(15),
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example_system)
        .run();

    // let expression = "1.0 + 1.2i";
    // let mut ctx = ComplexMathContext::new();
    // ctx.set_var("z", Complex64::new(0.3, 0.2));

    // let res = ComplexMath::calculate_expr(&mut ctx, expression).unwrap();

    // println!("Result: {}", res);

    Ok(())
}

fn setup(mut commands: Commands) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 2.0;

    commands.spawn((Camera2d, projection));
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

        ui.add(egui::Slider::new(&mut state.abs_spec, 0.0..=10.0).text("spec"));
        ui.add(egui::Slider::new(&mut state.abs_grid, 0.0..=10.0).text("grid"));
        ui.add(egui::Slider::new(&mut state.grid_step, 0.01..=1.0).text("grid_step"));

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.color_edit_button_srgba(&mut state.path_color);
            ui.color_edit_button_srgba(&mut state.background_color);
        });

        ui.separator();

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("Load SVG").clicked() {
                if let Some(file) = tinyfiledialogs::open_file_dialog("Open", "password.txt", None)
                {
                    state.loaded_svg_pah = Some(file.clone());
                    let svg_data = std::fs::read(file).unwrap();

                    let _g_code = generate_gcode(svg_data.clone()).unwrap();

                    let svg = Svg::from_bytes(&svg_data, "", Option::<&str>::None).unwrap();

                    let mesh = svg.tessellate();

                    let (w, h) = (svg.size.x, svg.size.y);

                    let ent = commands.spawn((
                        Mesh2d(meshes.add(Rectangle::new(w, h))),
                        MeshMaterial2d(materials.add(Color::srgba(
                            state.background_color.r() as f32 / 255.,
                            state.background_color.g() as f32 / 255.,
                            state.background_color.b() as f32 / 255.,
                            state.background_color.a() as f32 / 255.,
                        ))),
                        Transform::default().with_translation(Vec3::from_array([0.0, 0.0, -1.0])),
                    ));

                    state.entities.push(ent.id());

                    let ent = commands.spawn((
                        Mesh2d(meshes.add(mesh)),
                        MeshMaterial2d(materials.add(Color::srgba(
                            state.path_color.r() as f32 / 255.,
                            state.path_color.g() as f32 / 255.,
                            state.path_color.b() as f32 / 255.,
                            state.path_color.a() as f32 / 255.,
                        ))),
                        Transform::default().with_translation(Vec3::from_array([
                            -w / 2.0,
                            h / 2.0,
                            0.0,
                        ])),
                    ));

                    state.entities.push(ent.id());
                }
            }

            if ui.button("Compute").clicked() {
                println!("Button clicked: {}", state.expression);

                let value = state.expression.as_str();
                // let mut ctx = HashMap::new();

                let mut ctx = ComplexMathContext::new();

                // ctx.insert("pi", Complex64::new(std::f64::consts::PI, 0.0));
                // ctx.insert("e", Complex64::new(std::f64::consts::E, 0.0));
                // ctx.insert("i", Complex64::i());

                let abs_spec = state.abs_spec;
                let abs_grid = state.abs_grid;

                let svg_data = generate_graph(
                    -abs_spec..abs_spec,
                    -abs_spec..abs_spec,
                    Complex64::new(-abs_grid, -abs_grid),
                    Complex64::new(abs_grid, abs_grid),
                    state.grid_step,
                    |z| {
                        ctx.set_var("z", z);
                        ComplexMath::calculate_expr(&mut ctx, value).unwrap()
                    },
                )
                .unwrap();

                let _g_code = generate_gcode(svg_data.clone()).unwrap();

                let svg =
                    Svg::from_bytes(&svg_data, "plot_example.svg", Option::<&str>::None).unwrap();

                let mesh = svg.tessellate();

                let (w, h) = (svg.size.x, svg.size.y);

                let ent = commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(w, h))),
                    MeshMaterial2d(materials.add(Color::srgba(
                        state.background_color.r() as f32 / 255.,
                        state.background_color.g() as f32 / 255.,
                        state.background_color.b() as f32 / 255.,
                        state.background_color.a() as f32 / 255.,
                    ))),
                    Transform::default().with_translation(Vec3::from_array([0.0, 0.0, -1.0])),
                ));

                state.entities.push(ent.id());

                let ent = commands.spawn((
                    Mesh2d(meshes.add(mesh)),
                    MeshMaterial2d(materials.add(Color::srgba(
                        state.path_color.r() as f32 / 255.,
                        state.path_color.g() as f32 / 255.,
                        state.path_color.b() as f32 / 255.,
                        state.path_color.a() as f32 / 255.,
                    ))),
                    Transform::default().with_translation(Vec3::from_array([
                        -w / 2.0,
                        h / 2.0,
                        0.0,
                    ])),
                ));

                state.entities.push(ent.id());
            }
            if ui.button("Clear").clicked() {
                for ent in state.entities.iter() {
                    commands.entity(*ent).despawn();
                }

                state.entities.clear();
            }
        });
    });
}

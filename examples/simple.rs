use bevy::{
    color::palettes::css::{BLUE, WHITE},
    prelude::*,
    render::{mesh::VertexAttributeValues, render_resource::Face},
};
use bevy_egui::EguiPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use dock_inspector_egui::{DockInsepctorCamera, DockInspectorPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(DockInspectorPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(WHITE.into()))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut cuboid = Cuboid::from_length(4.0).mesh().build();
    if let Some(VertexAttributeValues::Float32x3(normals)) =
        cuboid.attribute_mut(Mesh::ATTRIBUTE_NORMAL)
    {
        for normal in normals {
            normal[0] = -normal[0];
            normal[1] = -normal[1];
            normal[2] = -normal[2];
        }
    }

    commands.spawn((
        Mesh3d(meshes.add(cuboid)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: BLUE.into(),
            cull_mode: Some(Face::Front),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
        DockInsepctorCamera,
    ));
}

use bevy::{math::vec3, prelude::*};
use bevy_trait_query::One;

use crate::floating_cam::controls::ControlState;

use self::{
    designer::{Designer, RegisterTraitPlugin},
    interaction::{interaction_designer::InteractionDesigner, InteractionDesignerPlugin},
    matrix::{matrix_designer::MatrixDesigner, MatrixDesignerPlugin},
};
pub mod designer;
pub mod interaction;
mod matrix;

pub struct DesignerPlugin;
impl Plugin for DesignerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RegisterTraitPlugin);

        let mut ds = DesignerStates::new();
        // ds.spawn_list.push(DesignerType::Interaction((
        //     5,
        //     vec3(0.0, 0.0, -5.0),
        //     vec3(5.0, 0.0, 0.0),
        //     0.5,
        //     0.05,
        // )));
        ds.spawn_list.push(DesignerType::Matrix((
            3,
            vec3(0.0, 0.0, -5.0),
            vec3(10.0, 10.0, 1.0),
        )));

        app.insert_resource(ds)
            .add_plugin(InteractionDesignerPlugin)
            .add_plugin(MatrixDesignerPlugin)
            .add_startup_system(spawn_designers)
            .add_system(update_designer);
    }
}

enum DesignerType {
    Interaction((usize, Vec3, Vec3, f32, f32)),
    Matrix((usize, Vec3, Vec3)),
}
impl DesignerType {
    pub fn spawn_designer(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> (Entity, Vec<Entity>) {
        let mut extra_data = vec![];
        let designer_entity = match self {
            DesignerType::Interaction((
                num_points,
                translation,
                size,
                point_radius,
                line_radius,
            )) => {
                let designer = InteractionDesigner::new(
                    num_points.clone(),
                    translation.clone(),
                    size.clone(),
                    point_radius.clone(),
                    line_radius.clone(),
                    commands,
                    asset_server,
                    meshes,
                    materials,
                );
                commands.spawn(designer).id()
            }
            DesignerType::Matrix((num_particles, translation, size)) => {
                let designer = MatrixDesigner::new(
                    num_particles.clone(),
                    translation.clone(),
                    size.clone(),
                    commands,
                    asset_server,
                    meshes,
                    materials,
                );
                extra_data = designer.cell_values.clone().into_iter().flatten().collect();
                commands.spawn(designer).id()
            }
        };
        return (designer_entity, extra_data);
    }
}

#[derive(Resource)]
pub struct DesignerStates {
    pub designers: Vec<Entity>,
    pub cur_designer: isize,
    spawn_list: Vec<DesignerType>,
}
impl DesignerStates {
    pub fn new() -> Self {
        return Self {
            designers: vec![],
            cur_designer: 0,
            spawn_list: vec![],
        };
    }

    pub fn despawn_designer(&mut self, i: usize, mut commands: Commands) {
        let despawn_entity = self.designers.remove(i);
        commands.entity(despawn_entity).despawn();
    }

    pub fn get_current_designer_entity(&self) -> Entity {
        return self.designers[self.cur_designer as usize];
    }
}

fn update_designer(
    designer_states: ResMut<DesignerStates>,
    mut control_state: ResMut<ControlState>,
    mut designers: Query<One<&mut dyn Designer>>,
) {
    if designer_states.cur_designer == -1 {
        return;
    }

    let mut designer =
        if let Ok(designer) = designers.get_mut(designer_states.get_current_designer_entity()) {
            designer
        } else {
            panic!();
        };

    let sen = 0.25;

    designer.apply_primary_nav_delta(control_state.designer_primary_nav_delta * sen);
    designer.apply_secondary_nav_delta(control_state.designer_secondary_nav_delta);
    designer.apply_primary_interact(control_state.designer_primary_interact);
    designer.apply_secondary_interact(control_state.designer_secondary_interact);

    control_state.reset_designer();
}

pub fn spawn_designers(
    mut designer_states: ResMut<DesignerStates>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    while !designer_states.spawn_list.is_empty() {
        let designer = designer_states.spawn_list.pop().unwrap();
        let mut spawn_data =
            designer.spawn_designer(&mut commands, &asset_server, &mut meshes, &mut materials);
        designer_states.designers.push(spawn_data.0);
        designer_states.designers.append(&mut spawn_data.1);
    }
}

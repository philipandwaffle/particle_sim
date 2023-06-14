use crate::floating_cam::controls::ControlState;

use super::Trickles;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Root {
    vessel: Box<dyn Trickles + Send + Sync>,
}

impl Root {
    pub fn new() -> Self {
        return Self { vessel: todo!() };
    }
}
impl Trickles for Root {
    fn drip(&self) {
        todo!()
    }
}

pub struct Dreg {
    primary_nav: Vec2,
    secondary_nav: isize,
    primary_interact: bool,
    secondary_interact: bool,
}

fn update_root(
    root: ResMut<Root>,
    mut control_state: ResMut<ControlState>,
    mut designers: Query<&mut dyn Trickles>,
) {
    root.drip(control_state);
    if root.cur_designer == -1 {
        return;
    }

    let mut root = if let Ok(designer) = designers.get_mut(root.drip()) {
        designer
    } else {
        panic!();
    };

    let sen = 0.25;

    root.apply_primary_nav_delta(control_state.designer_primary_nav_delta * sen);
    root.apply_secondary_nav_delta(control_state.designer_secondary_nav_delta);
    root.apply_primary_interact(control_state.designer_primary_interact);
    root.apply_secondary_interact(control_state.designer_secondary_interact);

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

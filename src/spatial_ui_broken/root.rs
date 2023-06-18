use std::cell::RefCell;

use crate::floating_cam::controls::ControlState;

use super::Trickles;
use bevy::prelude::*;
use bevy_trait_query::One;

#[derive(Resource)]
pub struct Root {
    contents: Entity,
}

impl Root {
    pub fn new(contents: Entity) -> Self {
        return Self { contents };
    }
}
impl Root {}
impl Trickles for Root {
    fn drip(&mut self, mut vessels: Query<One<&mut dyn Trickles>>, dreg: Dreg) {
        let mut vessel;
        match vessels.get_mut(self.contents) {
            Ok(v) => {
                vessel = v;
            }
            Err(err) => {
                panic!("attempted to get vessel that doesn't exist {}", err);
            }
        };
        vessel.drip(vessels, dreg);
    }
}

pub struct Dreg {
    pub primary_nav: Vec2,
    pub secondary_nav: isize,
    pub primary_interact: bool,
    pub secondary_interact: bool,
}
impl Dreg {
    pub fn new(
        primary_nav: Vec2,
        secondary_nav: isize,
        primary_interact: bool,
        secondary_interact: bool,
    ) -> Self {
        return Self {
            primary_nav,
            secondary_nav,
            primary_interact,
            secondary_interact,
        };
    }
}

pub fn update_root(
    mut root: ResMut<Root>,
    mut control_state: ResMut<ControlState>,
    mut vessels: Query<One<&mut dyn Trickles>>,
) {
    let sen = 0.25;
    let dreg = Dreg::new(
        control_state.designer_primary_nav_delta * sen,
        control_state.designer_secondary_nav_delta,
        control_state.designer_primary_interact,
        control_state.designer_secondary_interact,
    );
    root.drip(vessels, dreg);

    control_state.reset_designer();
}

// pub fn spawn_designers(
//     mut designer_states: ResMut<DesignerStates>,
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     while !designer_states.spawn_list.is_empty() {
//         let designer = designer_states.spawn_list.pop().unwrap();
//         let mut spawn_data =
//             designer.spawn_designer(&mut commands, &asset_server, &mut meshes, &mut materials);
//         designer_states.designers.push(spawn_data.0);
//         designer_states.designers.append(&mut spawn_data.1);
//     }
// }

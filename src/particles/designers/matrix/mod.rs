use bevy::prelude::*;

use self::{cell::Cell, matrix_designer::MatrixDesigner};

mod cell;
pub mod matrix_designer;

pub struct MatrixDesignerPlugin;
impl Plugin for MatrixDesignerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_display).add_system(update_cell_color);
    }
}

fn update_display(
    mut designers: Query<&mut MatrixDesigner, Changed<MatrixDesigner>>,
    mut cells: Query<&mut Cell>,
) {
    for mut designer in designers.iter_mut() {
        let cur = designer.cur_edit_point;
        let prev = designer.prev_edit_point;
        if cur == prev {
            continue;
        }

        let cur_cell_entity = designer.cell_entities[cur.x as usize][cur.y as usize];
        cells.get_mut(cur_cell_entity).unwrap().color = Color::GREEN;

        let prev_cell_entity = designer.cell_entities[prev.x as usize][prev.y as usize];
        cells.get_mut(prev_cell_entity).unwrap().color = Color::RED;

        designer.prev_edit_point = designer.cur_edit_point;
    }
}

fn update_cell_color(
    cells: Query<(&Handle<StandardMaterial>, &Cell), Changed<Cell>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (cell_material_handle, cell) in cells.iter() {
        let mut cell_material = materials.get_mut(&cell_material_handle).unwrap();
        cell_material.base_color = cell.color;
    }
}

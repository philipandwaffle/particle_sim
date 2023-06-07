use std::{sync::Arc, usize};

use bevy::{math::vec2, prelude::Resource};

use crate::particles::interaction_rule::interaction::CompThreshRule;

use super::interaction::{InteractionRule, SubRule, ZeroRule};

#[derive(Resource)]
pub struct InteractionMatrix {
    pub matrix: Vec<Vec<Box<dyn InteractionRule + Send + Sync>>>,
}
impl InteractionMatrix {
    pub fn new(count: usize) -> Self {
        let mut m = vec![];
        for i in 0..count {
            let mut row = vec![];
            for j in 0..count {
                // println!("row: {} col: {}", i, j);
                let cell = Box::new(ZeroRule) as Box<dyn InteractionRule + Send + Sync>;
                row.push(cell);
            }
            m.push(row);
        }
        // Box::clone(&foo);
        m[0][0] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, -1.0),
            vec2(2.0, 0.1),
        ]));
        m[0][1] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, -0.5),
            vec2(3.0, 0.5),
        ]));
        m[2][2] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, -1.0),
            vec2(2.0, 0.1),
        ]));
        m[2][3] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, -0.5),
            vec2(3.0, 0.5),
        ]));
        m[4][4] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, -1.0),
            vec2(2.0, 0.1),
        ]));
        m[4][5] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, -0.5),
            vec2(3.0, 0.5),
        ]));

        m[1][1] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, 1.0),
            vec2(3.0, 0.0),
        ]));
        m[1][3] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, 1.0),
            vec2(5.0, 0.0),
        ]));
        m[1][2] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, 1.0),
            vec2(3.0, 0.0),
        ]));

        m[3][3] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, 1.0),
            vec2(3.0, 0.0),
        ]));
        m[3][5] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, 1.0),
            vec2(5.0, 0.0),
        ]));
        m[3][4] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, 1.0),
            vec2(3.0, 0.0),
        ]));

        m[5][5] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, 1.0),
            vec2(3.0, 0.0),
        ]));
        m[5][1] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, 1.0),
            vec2(5.0, 0.0),
        ]));
        m[5][0] = Box::new(CompThreshRule::from_points(vec![
            vec2(0.0, 1.0),
            vec2(3.0, 0.0),
        ]));

        return Self { matrix: m };
    }
    pub fn get_interaction(&self, row: usize, col: usize, d: f32) -> f32 {
        if self.matrix.len() == 0 || self.matrix[row].len() == 0 {
            return 0.0;
        }
        return (self.matrix[row][col]).interact(d);
    }
}

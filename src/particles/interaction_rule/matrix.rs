use std::{sync::Arc, usize};

use bevy::{math::vec2, prelude::Resource};

use crate::particles::interaction_rule::interaction::CompThreshRule;

use super::interaction::{InteractionRule, ZeroRule};

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
                let cell = Box::new(CompThreshRule::from_points(vec![
                    vec2(0.0, -2.0),
                    vec2(2.0, 0.5),
                    vec2(7.0, 0.5),
                ])) as Box<dyn InteractionRule + Send + Sync>;
                row.push(cell);
            }
            m.push(row);
        }
        // Box::clone(&foo);
        return Self { matrix: m };
    }
    pub fn get_interaction(&self, row: usize, col: usize, d: f32) -> f32 {
        if self.matrix.len() == 0 || self.matrix[row].len() == 0 {
            return 0.0;
        }
        return (self.matrix[row][col]).interact(d);
    }
}

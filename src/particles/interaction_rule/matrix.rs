use std::{sync::Arc, usize};

use bevy::{math::vec2, prelude::Resource};

use crate::particles::interaction_rule::interaction::CompThreshRule;

use super::interaction::{InteractionRule, ZeroRule};

type IR = dyn InteractionRule + Send + Sync;

// trait IRClone {
//     fn clone_box(&self) -> Box<IR>;
// }

// impl<T> IRClone for T
// where
//     T: 'static + IR + Clone,
// {
//     fn clone_box(&self) -> Box<IR> {
//         Box::new(self.clone())
//     }
// }
// impl Clone for Box<IR> {
//     fn clone(&self) -> Box<IR> {
//         self.clone_box()
//     }
// }

#[derive(Resource)]
pub struct InteractionMatrix {
    matrix: Vec<Vec<Box<dyn InteractionRule + Send + Sync>>>,
    pub loading: bool,
}
impl InteractionMatrix {
    pub fn new(count: usize) -> Self {
        let mut m = vec![vec![]];
        for i in 0..count {
            let mut row = vec![];
            for j in 0..count {
                println!("row: {} col: {}", i, j);
                row.push(Box::new(CompThreshRule::from_points(vec![
                    vec2(0.0, 0.5),
                    vec2(7.0, 0.5),
                ]))
                    as Box<dyn InteractionRule + Send + Sync>)
            }
            m.push(row);
        }
        // Box::clone(&foo);
        return Self {
            matrix: m,
            loading: false,
        };
    }
    pub fn get_interaction(&self, row: usize, col: usize, d: f32) -> f32 {
        if self.matrix.len() == 0 || self.matrix[row].len() == 0 {
            return 0.0;
        }
        return (self.matrix[row][col]).interact(d);
    }
}

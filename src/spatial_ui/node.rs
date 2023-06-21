use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct Node {
    pub leaves: Option<Vec<Node>>,
    pub entity: Entity,
    pub enabled: bool,
}
impl Node {
    pub fn new(content: Entity) -> Self {
        return Self {
            leaves: None,
            entity: content,
            enabled: false,
        };
    }

    pub fn add_leaf(&mut self, content: Entity) {
        let leaf = Node::new(content);
        match self.leaves.as_mut() {
            Some(leaves) => leaves.push(leaf),
            None => self.leaves = Some(vec![leaf]),
        }
    }

    pub fn get_leaves(&self) -> Vec<Entity> {
        match &self.leaves {
            Some(leaves) => {
                let mut result = vec![self.entity];
                for leaf in leaves {
                    result.append(&mut leaf.get_leaves());
                }
                return result;
            }
            None => return vec![self.entity],
        }
    }
}

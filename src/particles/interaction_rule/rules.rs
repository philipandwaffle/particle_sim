use std::{rc::Rc, vec};

use bevy::{math::vec2, prelude::Vec2};

/// Generates an rule in the form  y = mx + c
macro_rules! linear_rule {
    ($d:tt, $m:expr, $c:expr) => {
        move |$d: f32| -> f32 {
            return $m * $d + $c;
        }
    };
}
/// Generates an rule in the form  y = c
macro_rules! constant_rule {
    ($c:expr) => {
        move |_: f32| -> f32 {
            return $c;
        }
    };
}

type RuleFunc = dyn Fn(f32) -> f32;

/// Contains function that is called when determining interaction
pub trait InteractionRule {
    // Takes a distance and returns a float to scale an interaction
    fn interact(&self, d: f32) -> f32;
}

// trait InteractionRuleClone {
//     fn clone_rc(&self) -> dyn InteractionRule;
// }
// impl<T> InteractionRuleClone for T
// where
//     T: 'static + InteractionRule + Clone,
// {
//     fn clone_rc(&self) -> Rc<dyn InteractionRule> {
//         Rc::new(self.clone())
//     }
// }
// impl Clone for Rc<dyn InteractionRule> {
//     fn clone(&self) -> dyn InteractionRule {
//         self.clone_rc()
//     }
// }

struct Simple;
impl Clone for Simple {
    fn clone(&self) -> Self {
        Self {}
    }
}

// Composed rules are comprised of smaller SubRules
#[derive(Clone)]
struct SubRule {
    rule: Rc<RuleFunc>,
}

impl SubRule {
    // Create a new linear sub rule
    fn new_linear(m: f32, c: f32) -> Self {
        return Self {
            rule: Rc::new(linear_rule!(x, m, c)),
        };
    }

    // Create a new constant sub rule
    fn new_constant(c: f32) -> Self {
        return Self {
            rule: Rc::new(constant_rule!(c)),
        };
    }
}
impl InteractionRule for SubRule {
    fn interact(&self, d: f32) -> f32 {
        let foo = 0;
        return (self.rule)(d);
    }
}

#[derive(Clone)]
pub struct ZeroRule;
impl ZeroRule {
    pub fn new() -> Self {
        return Self;
    }
}
impl InteractionRule for ZeroRule {
    fn interact(&self, _: f32) -> f32 {
        return 0.0;
    }
}
// A composite threshold rule consists of sub rules that are applied to across thresholds
#[derive(Clone)]
pub struct CompThreshRule {
    sub_rules: Vec<SubRule>,
    thresholds: Vec<f32>,
    count: usize,
    default_val: f32,
}
impl CompThreshRule {
    /// Creates a composite rule based on the vertices of a line on a 2d plane
    pub fn from_points(points: Vec<Vec2>) -> Self {
        let num_points = points.len();

        let mut sub_rules = Vec::with_capacity(num_points - 1);
        let mut thresholds = Vec::with_capacity(num_points - 1);

        for i in 0..num_points - 1 {
            sub_rules.push(Self::linear_sub_rule(points[i], points[i + 1]));
            thresholds.push(points[i + 1].x);
        }
        println!("{:?}", thresholds);

        return Self {
            sub_rules,
            thresholds,
            count: num_points - 1,
            default_val: points[num_points - 1].y,
        };
    }

    fn linear_sub_rule(a: Vec2, b: Vec2) -> SubRule {
        let rise = b.y - a.y;
        // special case when there is a horizontal line

        let run = b.x - a.x;
        // if run == 0.0 {
        //     return SubRule::new(1.0, c);
        // }

        let m = rise / run;
        let c = b.y - m * b.x;

        println!("y={}x+{} for:{} and {}", m, c, a, b);
        println!("rise: {}, run: {}", rise, run);

        return SubRule::new_linear(m, c);
    }
}
impl InteractionRule for CompThreshRule {
    fn interact(&self, d: f32) -> f32 {
        for i in 0..self.count {
            if d < self.thresholds[i] {
                return self.sub_rules[i].interact(d);
            }
        }
        return self.default_val;
    }
}

pub fn development() {
    let points = vec![
        vec2(0.0, 1.0),
        vec2(1.0, 0.5),
        vec2(2.0, 1.0),
        vec2(3.0, 0.0),
    ];
    let rule = CompThreshRule::from_points(points);

    // println!("{:?}", rule);
    for num in 0..10 {
        let d = num as f32 / 2.0;
        println!(
            "distance: {} \t res: {}",
            d,
            rule.interact(num as f32 / 2.0)
        );
    }
    // let foo = Box::new(sub_rule!(m, c));
    // let bar = fn(x:f32)->f32{

    // }
    // println!("{:?}", foo(10.0));
    // test_single_arg(10);
}

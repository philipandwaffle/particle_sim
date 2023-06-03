use std::vec;

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

/// Contains function that is called when determining interaction
pub trait InteractionRule {
    // Takes a distance and returns a float to scale an interaction
    fn interact(&self, d: f32) -> f32;
}

// Composed rules are comprised of smaller SubRules
struct SubRule<'a> {
    rule: Box<dyn Fn(f32) -> f32 + 'a + Send + Sync>,
}
impl SubRule<'_> {
    // Create a new linear sub rule
    fn new_linear(m: f32, c: f32) -> Self {
        return Self {
            rule: Box::new(linear_rule!(x, m, c)),
        };
    }

    // Create a new constant sub rule
    fn new_constant(c: f32) -> Self {
        return Self {
            rule: Box::new(constant_rule!(c)),
        };
    }
}
impl InteractionRule for SubRule<'_> {
    fn interact(&self, d: f32) -> f32 {
        return (self.rule)(d);
    }
}

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
pub struct CompThreshRule<'a> {
    sub_rules: Vec<SubRule<'a>>,
    thresholds: Vec<f32>,
    count: usize,
    default_val: f32,
}
impl CompThreshRule<'_> {
    /// Creates a composite rule based on the vertices of a line on a 2d plane
    pub fn from_points(points: Vec<Vec2>) -> Self {
        let num_points = points.len();

        let mut sub_rules = Vec::with_capacity(num_points - 1);
        let mut thresholds = Vec::with_capacity(num_points - 1);

        for i in 0..num_points - 1 {
            sub_rules.push(Self::linear_sub_rule(points[i], points[i + 1]));
            thresholds.push(points[i + 1].x);
        }

        return Self {
            sub_rules,
            thresholds,
            count: num_points - 1,
            default_val: points[num_points - 1].y,
        };
    }

    fn linear_sub_rule(a: Vec2, b: Vec2) -> SubRule<'static> {
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
impl InteractionRule for CompThreshRule<'_> {
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

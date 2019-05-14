#[macro_use]
extern crate serde_derive;

extern crate js_sys;
extern crate kdtree;
extern crate wasm_bindgen;
extern crate web_sys;

use js_sys::WebAssembly;
use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
struct Vector2d(f32, f32);

impl Vector2d {
    fn sub(mut self, other_vector: Vector2d) -> Vector2d {
        self.0 -= other_vector.0;
        self.1 -= other_vector.1;
        self
    }

    fn add(mut self, other_vector: Vector2d) -> Vector2d {
        self.0 += other_vector.0;
        self.1 += other_vector.1;
        self
    }

    fn div_n(mut self, n: f32) -> Vector2d {
        if n == 0.0 {
            return self;
        }

        self.0 /= n;
        self.1 /= n;
        self
    }

    fn mul_n(mut self, n: f32) -> Vector2d {
        self.0 *= n;
        self.1 *= n;
        self
    }

    fn magnitude(self) -> f32 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    fn normalize(self) -> Vector2d {
        let mag = self.magnitude();
        self.div_n(mag)
    }
}

#[derive(Deserialize, Debug, Clone)]
struct Point {
    pos: Vector2d,
    vel: Vector2d,
}

#[derive(Deserialize, Debug)]
struct Params {
    f: f32,
    r: f32,
}

#[derive(Deserialize, Debug)]
struct BehaviourNode(String, Params);

#[wasm_bindgen]
pub struct Simulation {
    behaviours: Vec<BehaviourNode>,
    points: Vec<Point>,
    tree: KdTree<f32, usize, [f32; 2]>,
}

fn tree_from_points(points: &[Point]) -> KdTree<f32, usize, [f32; 2]> {
    let mut tree = KdTree::new(2);

    for (i, point) in points.iter().enumerate() {
        tree.add([point.pos.0 as f32, point.pos.1 as f32], i)
            .unwrap();
    }

    tree
}

#[wasm_bindgen]
impl Simulation {
    pub fn create(points: &js_sys::Array, behaviours: &JsValue) -> Simulation {
        let behaviours: Vec<BehaviourNode> = behaviours.into_serde().unwrap();
        let points: Vec<(f32, f32)> = points.into_serde().unwrap();

        let mut final_points: Vec<Point> = [].to_vec();
        for point in points {
            final_points.push(Point {
                pos: Vector2d(point.0, point.1),
                vel: Vector2d(0.0, 0.0),
            })
        }

        // for node in &behaviours {
        //     log!("{:?}", node);
        // }

        // for point in points {
        //     log!("{:?}", point);
        // }

        let kdtree = tree_from_points(&final_points);

        Simulation {
            points: final_points,
            behaviours,
            tree: kdtree,
        }
    }

    pub fn step(&mut self) {
        for i in 0..self.points.len() {
            let mut new_vel = Vector2d(self.points[i].vel.0, self.points[i].vel.1);
            let current_pos = self.points[i].pos;

            for behaviour in &self.behaviours {
                if behaviour.0 == "repel" {
                    let nearby_points = self
                        .tree
                        .within(
                            &[self.points[i].pos.0 as f32, self.points[i].pos.1 as f32],
                            behaviour.1.r,
                            &squared_euclidean,
                        )
                        .unwrap();

                    for (_, nearby_idx) in nearby_points {
                        let vel_mod = current_pos
                            .sub(self.points[*nearby_idx].pos)
                            .normalize()
                            .mul_n(behaviour.1.f);

                        new_vel = new_vel.add(vel_mod);
                    }
                }

                if behaviour.0 == "dampen" {
                    new_vel = new_vel.mul_n(1.0 - behaviour.1.f);
                }
            }

            self.points[i].vel = new_vel;
            self.points[i].pos = self.points[i].pos.add(self.points[i].vel);
        }

        self.tree = tree_from_points(&self.points);
    }

    pub fn get(&self) -> js_sys::Float32Array {
        let points_flat = self.points.iter().fold(Vec::new(), |mut values, p| {
            values.push(p.pos.0);
            values.push(p.pos.1);
            values
        });

        let points: &[f32] = &points_flat;

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let points_location = points.as_ptr() as u32 / 4;

        js_sys::Float32Array::new(&memory_buffer)
            .subarray(points_location, points_location + points.len() as u32)
    }
}

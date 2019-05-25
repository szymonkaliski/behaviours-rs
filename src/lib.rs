#[macro_use]
extern crate serde_derive;

extern crate js_sys;
extern crate kdtree;
extern crate wasm_bindgen;
extern crate web_sys;

use js_sys::WebAssembly;
use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
struct Vector2d(f32, f32);

impl Default for Vector2d {
    fn default() -> Self {
        Vector2d(0.0, 0.0)
    }
}

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

type MetaMap = HashMap<String, String>;

#[derive(Deserialize, Debug, Clone)]
struct Point {
    pos: Vector2d,
    vel: Vector2d,
    meta: MetaMap,
}

#[derive(Deserialize, Debug)]
struct Test(String, String, String);

impl Default for Test {
    fn default() -> Self {
        Test("NOP".to_string(), "".to_string(), "".to_string())
    }
}

#[derive(Deserialize, Debug)]
struct Params {
    f: Option<f32>,
    r: Option<f32>,
    p: Option<Vector2d>,
    test: Option<Test>,
    key: Option<String>,
    value: Option<String>,
}

#[derive(Deserialize, Debug)]
struct BehaviourNode {
    behaviour: String,
    params: Params,

    #[serde(default)]
    children: Vec<BehaviourNode>,
}

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
                meta: MetaMap::new(),
            })
        }

        // for b in &behaviours {
        //     log!("{:?}", b);
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

    #[wasm_bindgen(js_name = setMeta)]
    pub fn set_meta(&mut self, idx: usize, key: String, value: String) {
        self.points[idx].meta.insert(key, value);
    }

    fn process_behaviours(
        &self,
        point: &Point,
        behaviours: &[BehaviourNode],
    ) -> (Vector2d, MetaMap) {
        let mut vel = Vector2d(point.vel.0, point.vel.1);
        let mut meta = point.meta.clone();

        let empty_value = "".to_string();

        for b in behaviours {
            if b.behaviour == "if" {
                let test = b.params.test.as_ref().unwrap();

                let op = &test.0;
                let key = &test.1;
                let test_value = &test.2;

                let point_value = point.meta.get(key).unwrap_or(&empty_value);

                if (op == "!=" && test_value != point_value)
                    || (op == "==" && test_value == point_value)
                {
                    let (child_vel, child_meta) = self.process_behaviours(point, &b.children);

                    // FIXME: not sure about this, why `vel = vel.add(child_vel);` doesn't work?
                    vel = child_vel;
                    meta.extend(child_meta);
                }
            }

            if b.behaviour == "repel" {
                let nearby_points = self
                    .tree
                    .within(
                        &[point.pos.0 as f32, point.pos.1 as f32],
                        b.params.r.unwrap_or(0.0),
                        &squared_euclidean,
                    )
                    .unwrap();

                for (_, nearby_idx) in nearby_points {
                    let vel_mod = point
                        .pos
                        .sub(self.points[*nearby_idx].pos)
                        .normalize()
                        .mul_n(b.params.f.unwrap_or(0.0));

                    vel = vel.add(vel_mod);
                }
            }

            if b.behaviour == "attract" {
                let p = b.params.p.unwrap_or_default();

                let should_impact = if b.params.r.unwrap_or(0.0) != 0.0 {
                    squared_euclidean(&[p.0, p.1], &[point.pos.0, point.pos.1])
                        < b.params.r.unwrap_or(0.0)
                } else {
                    true
                };

                if should_impact {
                    let vel_mod = p
                        .sub(point.pos)
                        .normalize()
                        .mul_n(b.params.f.unwrap_or(0.0));

                    vel = vel.add(vel_mod);
                }
            }

            if b.behaviour == "dampen" {
                vel = vel.mul_n(1.0 - b.params.f.unwrap_or(0.0));
            }

            if b.behaviour == "collide" {
                let test = b.params.test.as_ref().unwrap();

                let op = &test.0;
                let key = &test.1;
                let test_value = &test.2;

                let nearby_points = self
                    .tree
                    .within(
                        &[point.pos.0 as f32, point.pos.1 as f32],
                        b.params.r.unwrap_or(0.0),
                        &squared_euclidean,
                    )
                    .unwrap();

                let mut did_collide_passing_test = false;

                for (_, nearby_idx) in nearby_points {
                    let point_value = self.points[*nearby_idx]
                        .meta
                        .get(key)
                        .unwrap_or(&empty_value);

                    if (op == "!=" && test_value != point_value)
                        || (op == "==" && test_value == point_value)
                    {
                        did_collide_passing_test = true;
                        break;
                    }
                }

                if did_collide_passing_test {
                    let (child_vel, child_meta) = self.process_behaviours(point, &b.children);

                    // FIXME: not sure about this, why `vel = vel.add(child_vel);` doesn't work?
                    vel = child_vel;
                    meta.extend(child_meta);
                }
            }

            if b.behaviour == "set" {
                match (&b.params.key, &b.params.value) {
                    (Some(key), Some(value)) => meta.insert(key.clone(), value.clone()),
                    _ => None,
                };
            }

            if b.behaviour == "stop" {
                vel = Vector2d(0.0, 0.0);
            }
        }

        (vel, meta)
    }

    pub fn step(&mut self) {
        for i in 0..self.points.len() {
            let (new_vel, new_meta) = self.process_behaviours(&self.points[i], &self.behaviours);

            self.points[i].vel = new_vel;
            self.points[i].pos = self.points[i].pos.add(self.points[i].vel);
            self.points[i].meta.extend(new_meta);
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

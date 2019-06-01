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
struct Vector {
    x: f32,
    y: f32,
    #[serde(default)]
    z: f32,
}

impl Default for Vector {
    fn default() -> Self {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Vector {
    fn sub(mut self, other_vector: Vector) -> Vector {
        self.x -= other_vector.x;
        self.y -= other_vector.y;
        self.z -= other_vector.z;
        self
    }

    fn add(mut self, other_vector: Vector) -> Vector {
        self.x += other_vector.x;
        self.y += other_vector.y;
        self.z += other_vector.z;
        self
    }

    fn div_n(mut self, n: f32) -> Vector {
        if n == 0.0 {
            return self;
        }

        self.x /= n;
        self.y /= n;
        self.z /= n;
        self
    }

    fn mul_n(mut self, n: f32) -> Vector {
        self.x *= n;
        self.y *= n;
        self.z *= n;
        self
    }

    fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(self) -> Vector {
        let mag = self.magnitude();
        self.div_n(mag)
    }
}

type MetaMap = HashMap<String, String>;

#[derive(Deserialize, Debug, Clone)]
struct Point {
    pos: Vector,
    vel: Vector,
    meta: MetaMap,
}

#[derive(Deserialize, Debug)]
struct Test {
    op: String,
    key: String,
    value: String,
}

impl Default for Test {
    fn default() -> Self {
        Test {
            op: "NOP".to_string(),
            key: "".to_string(),
            value: "".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Params {
    f: Option<f32>,
    r: Option<f32>,
    p: Option<Vector>,
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
    tree2d: Option<KdTree<f32, usize, [f32; 2]>>,
    tree3d: Option<KdTree<f32, usize, [f32; 3]>>,
    dims: usize,
}

fn tree2d_from_points(points: &[Point]) -> KdTree<f32, usize, [f32; 2]> {
    let mut tree = KdTree::new(2);

    for (i, point) in points.iter().enumerate() {
        tree.add([point.pos.x, point.pos.y], i).unwrap();
    }

    tree
}

fn tree3d_from_points(points: &[Point]) -> KdTree<f32, usize, [f32; 3]> {
    let mut tree = KdTree::new(3);

    for (i, point) in points.iter().enumerate() {
        tree.add([point.pos.x, point.pos.y, point.pos.z], i)
            .unwrap();
    }

    tree
}

#[wasm_bindgen]
impl Simulation {
    pub fn create(
        points_flat: &js_sys::Float32Array,
        dims: usize,
        behaviours: &JsValue,
    ) -> Simulation {
        let behaviours: Vec<BehaviourNode> = behaviours.into_serde().unwrap();

        // log!("[behaviours] {:?}", behaviours);

        let mut points_tmp = Vec::new();
        let mut final_points: Vec<Point> = [].to_vec();

        points_flat.for_each(&mut |n, _, _| points_tmp.push(n));

        for i in 0..points_tmp.len() / dims {
            let x = points_tmp[i * dims];
            let y = points_tmp[i * dims + 1];
            let z = if dims == 2 {
                0.0
            } else {
                points_tmp[i * dims + 2]
            };

            final_points.push(Point {
                pos: Vector { x, y, z },
                vel: Vector::default(),
                meta: MetaMap::new(),
            })
        }

        // log!("[final_points] {:?}", final_points);

        let tree2d = if dims == 2 {
            Some(tree2d_from_points(&final_points))
        } else {
            None
        };

        let tree3d = if dims == 3 {
            Some(tree3d_from_points(&final_points))
        } else {
            None
        };

        Simulation {
            points: final_points,
            behaviours,
            tree2d,
            tree3d,
            dims,
        }
    }

    #[wasm_bindgen(js_name = setMeta)]
    pub fn set_meta(&mut self, idx: usize, key: String, value: String) {
        self.points[idx].meta.insert(key, value);
    }

    #[wasm_bindgen(js_name = _replaceBehaviours)]
    pub fn replace_behaviors(&mut self, behaviours: &JsValue) {
        self.behaviours = behaviours.into_serde().unwrap();
    }

    fn vel_for_pos_or_others(&self, params: &Params, point: &Point) -> Vector {
        let mut vel_mod = Vector::default();

        match params.p {
            Some(p) => {
                let should_impact = if params.r.unwrap_or(0.0) != 0.0 {
                    let d = squared_euclidean(
                        &[p.x, p.y, p.z],
                        &[point.pos.x, point.pos.y, point.pos.z],
                    );

                    d < params.r.unwrap_or(0.0)
                } else {
                    true
                };

                if should_impact {
                    vel_mod = p.sub(point.pos).normalize().mul_n(params.f.unwrap_or(0.0));
                }
            }

            None => {
                let nearby_points = if self.dims == 2 {
                    self.tree2d
                        .as_ref()
                        .unwrap()
                        .within(
                            &[point.pos.x, point.pos.y],
                            params.r.unwrap_or(0.0),
                            &squared_euclidean,
                        )
                        .unwrap()
                } else {
                    self.tree3d
                        .as_ref()
                        .unwrap()
                        .within(
                            &[point.pos.x, point.pos.y, point.pos.z],
                            params.r.unwrap_or(0.0),
                            &squared_euclidean,
                        )
                        .unwrap()
                };

                for (_, nearby_idx) in nearby_points {
                    vel_mod = vel_mod.sub(
                        point
                            .pos
                            .sub(self.points[*nearby_idx].pos)
                            .normalize()
                            .mul_n(params.f.unwrap_or(0.0)),
                    );
                }
            }
        }

        vel_mod
    }

    fn process_behaviours(&self, point: &Point, behaviours: &[BehaviourNode]) -> (Vector, MetaMap) {
        let mut vel = point.vel.clone();
        let mut meta = point.meta.clone();

        let empty_value = "".to_string();

        for b in behaviours {
            if b.behaviour == "if" {
                let test_default = &Test::default();
                let test = b.params.test.as_ref().unwrap_or(test_default);

                let point_value = point.meta.get(&test.key).unwrap_or(&empty_value);

                if (test.op == "!=" && test.value != *point_value)
                    || (test.op == "==" && test.value == *point_value)
                {
                    let (child_vel, child_meta) = self.process_behaviours(point, &b.children);

                    // FIXME: not sure about this, why `vel = vel.add(child_vel);` doesn't work?
                    vel = child_vel;
                    meta.extend(child_meta);
                }
            }

            if b.behaviour == "repel" {
                let b_vel = self.vel_for_pos_or_others(&b.params, point);
                vel = vel.sub(b_vel);
            }

            if b.behaviour == "attract" {
                let b_vel = self.vel_for_pos_or_others(&b.params, point);
                vel = vel.add(b_vel);
            }

            if b.behaviour == "dampen" {
                vel = vel.mul_n(1.0 - b.params.f.unwrap_or(0.0));
            }

            if b.behaviour == "collide" {
                let test_default = &Test::default();
                let test = b.params.test.as_ref().unwrap_or(test_default);

                let nearby_points = if self.dims == 2 {
                    self.tree2d
                        .as_ref()
                        .unwrap()
                        .within(
                            &[point.pos.x, point.pos.y],
                            b.params.r.unwrap_or(0.0),
                            &squared_euclidean,
                        )
                        .unwrap()
                } else {
                    self.tree3d
                        .as_ref()
                        .unwrap()
                        .within(
                            &[point.pos.x, point.pos.y, point.pos.z],
                            b.params.r.unwrap_or(0.0),
                            &squared_euclidean,
                        )
                        .unwrap()
                };

                let mut did_collide_passing_test = test.value == "NOP".to_string();

                if !did_collide_passing_test {
                    for (_, nearby_idx) in nearby_points {
                        let point_value = self.points[*nearby_idx]
                            .meta
                            .get(&test.key)
                            .unwrap_or(&empty_value);

                        if (test.op == "!=" && test.value != *point_value)
                            || (test.op == "==" && test.value == *point_value)
                        {
                            did_collide_passing_test = true;
                            break;
                        }
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
                vel = Vector::default();
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

        if self.dims == 2 {
            self.tree2d = Some(tree2d_from_points(&self.points));
        } else {
            self.tree3d = Some(tree3d_from_points(&self.points));
        }
    }

    pub fn get(&self) -> js_sys::Float32Array {
        let points_flat = self.points.iter().fold(Vec::new(), |mut values, p| {
            values.push(p.pos.x);
            values.push(p.pos.y);
            if self.dims == 3 {
                values.push(p.pos.z);
            }

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

    #[wasm_bindgen(js_name = getIf)]
    pub fn get_if(&self, test: &JsValue) -> js_sys::Float32Array {
        let test: Test = test.into_serde().unwrap();

        let empty_value = "".to_string();

        let points_flat = self.points.iter().fold(Vec::new(), |mut values, p| {
            let point_value = p.meta.get(&test.key).unwrap_or(&empty_value);

            if (test.op == "!=" && test.value != *point_value)
                || (test.op == "==" && test.value == *point_value)
            {
                values.push(p.pos.x);
                values.push(p.pos.y);
                if self.dims == 3 {
                    values.push(p.pos.z);
                }
            }

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

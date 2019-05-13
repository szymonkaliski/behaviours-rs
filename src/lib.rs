#[macro_use]
extern crate serde_derive;

extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

extern crate js_sys;
extern crate wasm_bindgen;

use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Deserialize, Debug, Clone)]
struct Point {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
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
}

#[wasm_bindgen]
impl Simulation {
    pub fn create(points: &js_sys::Array, tree: &JsValue) -> Simulation {
        let tree: Vec<BehaviourNode> = tree.into_serde().unwrap();
        let points: Vec<(f32, f32)> = points.into_serde().unwrap();

        let mut final_points: Vec<Point> = [].to_vec();
        for point in points {
            final_points.push(Point {
                x: point.0,
                y: point.1,
                vx: 0.0,
                vy: 0.0,
            })
        }

        // for node in tree {
        //     log!("{:?}", node);
        // }

        // for point in points {
        //     log!("{:?}", point);
        // }

        Simulation {
            points: final_points,
            behaviours: tree,
        }
    }

    pub fn step(&mut self) {
        for i in 0..self.points.len() {
            for behaviour in &self.behaviours {
                if behaviour.0 == "repel" {
                    self.points[i].x += 1.0;
                    self.points[i].y += 0.0;
                }
            }
        }
    }

    pub fn get(&self) -> js_sys::Float32Array {
        let points_flat = self.points.iter().fold(Vec::new(), |mut values, p| {
            values.push(p.x);
            values.push(p.y);
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

#[macro_use]
extern crate lazy_static;

extern crate rand;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::sync::Mutex;
use rand::distributions::{Distribution, Uniform};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct Point2D {
    x: f64,
    y: f64,
    speed: f64,
    rgba: String
}

impl Point2D {
    fn new(x: f64, y: f64, speed: f64, rgba: String) -> Point2D {
        Point2D { x, y, speed, rgba }
    }

    pub fn update_y(&mut self) {
        if self.y > 350.0 {
            return self.y = 0.0;
        }

        self.y = self.y + self.speed;
    }
}

struct Points {
    points: Vec<Point2D>
}

impl Points {
    fn new() -> Points {
        Points { points: Vec::new() }
    }

    pub fn get_context(&mut self) -> web_sys::CanvasRenderingContext2d {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context
    }

    pub fn create_points(&mut self) {
        let die_xy = Uniform::from(1..350);
        let die_speed = Uniform::from(0.1..2.0);
        let mut rng = rand::thread_rng();

        for _ in 1..1000 {
            let x = die_xy.sample(&mut rng);
            let y = die_xy.sample(&mut rng);
            let speed = die_speed.sample(&mut rng);
            let opacity = 1.0 * speed / 2.0;
            let mut rgba = String::from("rgba(0,0,0,");
            rgba.push_str(&opacity.to_string());

            self.points.push(Point2D::new(x as f64, y as f64, speed, rgba))
        }
    }

    pub fn draw_points(&mut self) {
        let context: web_sys::CanvasRenderingContext2d = self.get_context();
        for point in &self.points {
            let rgba = String::from(point.rgba.clone());
            
            context.set_fill_style(&rgba.into());
            context.fill_rect(point.x, point.y, 5.0, 5.0);
        }
    }

    pub fn clear_points(&mut self) {
        let context: web_sys::CanvasRenderingContext2d = self.get_context();
        context.set_fill_style(&"#ffffff".into());
        context.fill_rect(0.0, 0.0, 350.0, 350.0);
    }

    pub fn update_points(&mut self) {
        for point in &mut self.points {
            point.update_y();
        }
    }
}

lazy_static! {
    static ref STATES: Mutex<Points> = Mutex::new(Points::new());
}

#[wasm_bindgen(start)]
pub fn start() {
    let mut points = STATES.lock().unwrap();
    points.create_points();
}

#[wasm_bindgen()]
pub fn refresh() {
    let mut points = STATES.lock().unwrap();
    points.clear_points();
    points.update_points();
    points.draw_points();
}
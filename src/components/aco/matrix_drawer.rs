use std::f64::INFINITY;

use dioxus::prelude::*;

use crate::utils::{ACOConfig, ACOPaths, MatrixACOPaths, Point2};

#[derive(Props, Clone)]
pub struct MatrixDrawerProps {
    matrix: Box<MatrixACOPaths>,
    id: i32
}

impl PartialEq for MatrixDrawerProps {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

const PADDING: f64 = 20.;
const RADIUS: f64 = 5.;
const HEIGHT: usize = 300;
const WIDTH: usize = 400;

#[derive(PartialEq)]
struct Dimensions {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64
}

#[derive(PartialEq)]
struct Circle {
    cx: f64,
    cy: f64,
}

#[derive(PartialEq)]
struct Line {
    a: Point2,
    b: Point2,
    strength: f64
}

#[component]
pub fn MatrixDrawer(props: MatrixDrawerProps) -> Element {
    let dimensions_matrix = props.matrix.clone();
    let dimensions = {
        let mut min_x = INFINITY;
        let mut max_x = -INFINITY;
        let mut min_y = INFINITY;
        let mut max_y = -INFINITY;

        for point in dimensions_matrix.points.iter() {
            if min_x > point.x {
                min_x = point.x;
            }
            if max_x < point.x {
                max_x = point.x;
            } 
            if min_y > point.y {
                min_y = point.y;
            }
            if max_y < point.y {
                max_y = point.y;
            } 
        }

        Dimensions {min_x, min_y, max_x, max_y}
    };

    let circles_matrix = props.matrix.clone();
    let circles = {
        let aspect_w = (WIDTH as f64 - 2. * PADDING) / (dimensions.max_x - dimensions.min_x);
        let aspect_h = (HEIGHT as f64 - 2. * PADDING) / (dimensions.max_y - dimensions.min_y);

        let mut result = Vec::new();

        for point in circles_matrix.points.iter() {
            result.push(Circle {
                cx: -dimensions.min_x * aspect_w + point.x * aspect_w, 
                cy: -dimensions.min_y * aspect_h + point.y * aspect_h
            });
        }

        result
    };

    let lines_matrix = props.matrix.clone();
    let lines = {
        let aspect_w = (WIDTH as f64 - 2. * PADDING) / (dimensions.max_x - dimensions.min_x);
        let aspect_h = (HEIGHT as f64 - 2. * PADDING) / (dimensions.max_y - dimensions.min_y);

        let mut result = Vec::new();
        let mut min_feromone = INFINITY;
        let mut max_feromone = -INFINITY;
        for i in 0..lines_matrix.len() {
            for j in 0..lines_matrix.len() {
                let feromone = lines_matrix.get_feromone_intensity(i, j).unwrap_or(0.);
                if feromone > max_feromone {
                    max_feromone = feromone;
                }
                if feromone < min_feromone {
                    min_feromone = feromone;
                }
            }
        }

        for i in 0..lines_matrix.len() {
            for j in 0..lines_matrix.len() {
                let point_a = lines_matrix.points.get(i).unwrap();
                let point_b = lines_matrix.points.get(j).unwrap();
                result.push(Line {
                    a: Point2 { 
                        x: -dimensions.min_x * aspect_w + point_a.x * aspect_w, 
                        y: -dimensions.min_y * aspect_h + point_a.y * aspect_h 
                    },
                    b: Point2 { 
                        x: -dimensions.min_x * aspect_w + point_b.x * aspect_w, 
                        y: -dimensions.min_y * aspect_h + point_b.y * aspect_h 
                    },
                    strength: (lines_matrix.get_feromone_intensity(i, j).unwrap_or(0.) - min_feromone) / (max_feromone - min_feromone)
                });
            }
        }

        result
    };
    
    rsx! {
        svg {
            class: "bg-white rounded-sm aspect-4/3",
            view_box: "0 0 400 300",
            xmlns: "http://www.w3.org/2000/svg",
            for line in lines.iter() {
                line { 
                    x1: line.a.x + PADDING as f64,
                    y1: line.a.y + PADDING as f64,
                    x2: line.b.x + PADDING as f64,
                    y2: line.b.y + PADDING as f64,
                    stroke: "yellow",
                    stroke_width: line.strength * 3.0
                }
            }
            for circle in circles.iter() {
                circle { 
                    cx: circle.cx + PADDING as f64,
                    cy: circle.cy + PADDING as f64,
                    r: RADIUS as f64,
                    fill: "red"
                }
            }
        }
    }
}
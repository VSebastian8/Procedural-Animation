use iced::{Point, Vector};

#[derive(Debug)]
pub enum Orientation {
    LEFT,
    CENTER,
    RIGHT,
}

pub fn point_to_vector(point: Point) -> Vector {
    Vector::new(point.x, point.y)
}

// Calculate the length of a 2D vector
pub fn vector_length(v: Vector) -> f32 {
    (v.x.powf(2.0) + v.y.powf(2.0)).sqrt()
}

// Calculate the distance between 2 points
pub fn point_distance(p1: Point, p2: Point) -> f32 {
    vector_length(p1 - p2)
}

// Calculate the angle between 2 vectors
pub fn angle_2_vectors(a: Vector, b: Vector) -> f32 {
    ((a.x * b.x + a.y * b.y)
        / ((a.x.powf(2.0) + a.y.powf(2.0)).sqrt() * (b.x.powf(2.0) + b.y.powf(2.0)).sqrt()))
    .acos()
}

// Determine wether c is Left, Right or Colinear with the vector from a to b
pub fn orientation_test(a: Point, b: Point, c: Point) -> Orientation {
    let det = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if det == 0.0 {
        Orientation::CENTER
    } else if det < 0.0 {
        Orientation::LEFT
    } else {
        Orientation::RIGHT
    }
}

// Rotate the vector v by a degrees
pub fn rotate_vector(v: Vector, a: f32) -> Vector {
    Vector::new(a.cos() * v.x - a.sin() * v.y, a.sin() * v.x + a.cos() * v.y)
}

#[allow(dead_code)]
// Function that calculates the circle passing through 3 points, returns circle center and radius
pub fn circle_from_three_points(a: Point, b: Point, c: Point) -> (Point, f32) {
    let xab = a.x - b.x;
    let xac = a.x - c.x;
    let yab = a.y - b.y;
    let yac = a.y - c.y;

    // Square difference
    let sqxac = a.x * a.x - c.x * c.x;
    let sqxba = b.x * b.x - a.x * a.x;
    let sqyac = a.y * a.y - c.y * c.y;
    let sqyba = b.y * b.y - a.y * a.y;

    let f =
        (sqxac * xab + sqyac * xab + sqxba * xac + sqyba * xac) / (2.0 * (yab * xac - yac * xab));
    let g =
        (sqxac * yab + sqyac * yab + sqxba * yac + sqyba * yac) / (2.0 * (xab * yac - xac * yab));
    let c = -a.x * a.x - a.y * a.y - 2.0 * g * a.x - 2.0 * f * a.y;
    let r = (f * f + g * g - c).sqrt();

    (Point::new(-g, -f), r)
}

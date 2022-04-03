use bevy::prelude::*;

use crate::game::player::Direction;

// square + circle
pub fn cmp_circle_and_square(
    circle_location: &Vec3,
    circle_radius: f32,
    square_location: &Vec3,
    square_apothem: f32,
) -> Option<Direction> {
    // first compare the player location center to the brick center
    let (x, y) = (
        circle_location.x - square_location.x,
        circle_location.y - square_location.y,
    );
    // then find the vector to the nearest brick corner by rotating coordinate system into Q1
    let (offset_x, offset_y) = (
        x.abs() - square_apothem,
        y.abs() - square_apothem,
    );
    // TODO: if x.abs() < 0. && y.abs() < 0. { return true; }
    // now check if the corner is within the player's circle
    if Vec2::new(offset_x, offset_y).length() < circle_radius {
        if x.is_sign_positive() && x >= y.abs() {
            Some(Direction::Left)
        } else if x.is_sign_negative() && -x >= y.abs() {
            Some(Direction::Right)
        } else if y.is_sign_positive() && y > x.abs() {
            Some(Direction::Bottom)
        } else {
            Some(Direction::Top)
        }
    } else {
        None
    }
}

// derived from http://www.phatcode.net/articles.php?id=459
pub fn cmp_circle_and_triangle(
    circle_location: &Vec3,
    circle_radius: f32,
    triangle_path: (Vec2, Vec2, Vec2),
) -> bool {
    // get relevant vectors
    let origin = Vec2::new(circle_location.x, circle_location.y);
    let (point1, point2, point3) = triangle_path;
    let edge1 = point2 - point1;
    let edge2 = point3 - point2;
    let edge3 = point1 - point3;
    let circle_to_v1 = origin - point1;
    let circle_to_v2 = origin - point2;
    let circle_to_v3 = origin - point3;

    // is any triangle vertex within the circle
    let cmp_circle_and_vertex = |circle_to_vertex: Vec2| {
        circle_to_vertex.dot(circle_to_vertex) - circle_location.length() <= 0.
    };
    if cmp_circle_and_vertex(point1) || cmp_circle_and_vertex(point2) || cmp_circle_and_vertex(point3) {
        return true
    }

    // is the circle center fully inside the triangle
    if circle_to_v1.perp_dot(edge1) >= 0. && circle_to_v2.perp_dot(edge2) >= 0. && circle_to_v3.perp_dot(edge3) >= 0. {
        return true
    }

    // does the circle intersect any edge
    let cmp_circle_and_edge = |circle_to_vertex: &Vec2, edge: &Vec2| {
        let alignment = circle_to_vertex.dot(*edge);
        alignment > 0. && alignment < edge.length_squared() && circle_to_vertex.length_squared() * edge.length_squared() <= alignment * alignment
    };
    if vec![(circle_to_v1, edge1), (circle_to_v2, edge2), (circle_to_v3, edge3)]
            .iter()
            .any(|(cv, edge)| cmp_circle_and_edge(cv, edge)) {
        return true
    }

    // nothing found
    return false
}

pub fn cmp_circles(attacker: Vec3, target: Vec3, radii: f32) -> bool {
    (attacker - target).length() < radii * 2.
}

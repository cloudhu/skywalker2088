//! Math Helper functions
use bevy::prelude::*;
use rand::{thread_rng, Rng};
pub struct Math;

impl Math {
    pub fn quaternion_from_2d_vector(vector: Vec2) -> Quat {
        let angle = vector.y.atan2(vector.x);
        Quat::from_rotation_z(angle)
    }

    pub fn random_2d_unit_vector() -> Vec2 {
        let mut rng = rand::thread_rng();
        Vec2 {
            x: rng.gen_range(-1.0..1.0),
            y: rng.gen_range(-1.0..1.0),
        }
        .try_normalize()
        .unwrap_or(Vec2::ONE)
    }

    fn point_is_on_line(point: Vec2, line_start: Vec2, line_end: Vec2) -> bool {
        const BUFFER: f32 = 0.1; // higher # = less accurate
        let d1 = point.distance(line_start);
        let d2 = point.distance(line_end);
        let line_length = line_start.distance(line_end);
        d1 + d2 >= line_length - BUFFER && d1 + d2 <= line_length + BUFFER
    }

    pub fn distance_from_point_to_line(point: Vec2, line_start: Vec2, line_end: Vec2) -> f32 {
        // Calculate the closest point "on the line"

        let line = line_end - line_start;
        let from_point_to_source = point - line_start;
        let dot = line.dot(from_point_to_source) / line.length_squared();
        let closest_x = line_start.x + dot * (line_end.x - line_start.x);
        let closest_y = line_start.y + dot * (line_end.y - line_start.y);
        // Check if on line
        let closest = Vec2 {
            x: closest_x,
            y: closest_y,
        };
        if Math::point_is_on_line(closest, line_start, line_end) {
            return closest.distance(point);
        }
        // Not on line so return closest end point
        let distance_to_start = closest.distance(line_start);
        let distance_to_end = closest.distance(line_end);
        if distance_to_start < distance_to_end {
            return distance_to_start;
        }
        distance_to_end
    }
}

/// Randomly picks index of vector using weights
/// Takes in a vector of weights
pub fn weighted_rng(probs: Vec<f32>) -> usize {
    let prob_space = probs.iter().fold(0.0, |sum, prob| sum + prob);
    let pos = thread_rng().random::<f32>() * prob_space;
    let mut sum = 0.0;
    for (idx, prob) in probs.iter().enumerate() {
        sum += prob;
        if sum > pos {
            return idx;
        }
    }
    unreachable!("Error in probabilities.");
}

/// Signed modulo function
pub fn signed_modulo(a: f32, n: f32) -> f32 {
    a - (a / n).floor() * n
}

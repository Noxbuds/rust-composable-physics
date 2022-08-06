use crate::{entity::{EntityContainer, Entity}, components::{position::Position, Component, circle_collider::CircleCollider}};
use super::System;

pub struct Collision;

fn handle_pair(filtered_entities: &mut Vec<&mut Entity>, i: usize, j: usize) -> Option<()> {
    let position_i = Position::get(&filtered_entities[i], 0)?;
    let collider_i = CircleCollider::get(&filtered_entities[i], 0)?;

    let position_j = Position::get(&filtered_entities[j], 0)?;
    let collider_j = CircleCollider::get(&filtered_entities[j], 0)?;

    let dir = position_i.to_vec2() - position_j.to_vec2();
    let radius_sum = collider_i.radius + collider_j.radius;
    let sqr_dist = dir.sqr_len();

    if sqr_dist < radius_sum * radius_sum {
        let dist = sqr_dist.sqrt();
        let axis = dir / dist;
        let delta = radius_sum - dist;

        let new_position_i = Position::from_vec2(position_i.to_vec2() + axis * 0.5 * delta);
        let new_position_j = Position::from_vec2(position_j.to_vec2() - axis * 0.5 * delta);

        Position::attach(&mut filtered_entities[i], new_position_i, 0);
        Position::attach(&mut filtered_entities[j], new_position_j, 0);
    }

    Some(())
}

impl System for Collision {
    fn apply(&mut self, entities: &mut EntityContainer, _dt: f64) {
        let mut filtered_entities: Vec<&mut Entity> = entities.values_mut().collect();

        for i in 0..filtered_entities.len() {
            for j in i + 1..filtered_entities.len() {
                handle_pair(&mut filtered_entities, i, j);
            }
        }
    }
}

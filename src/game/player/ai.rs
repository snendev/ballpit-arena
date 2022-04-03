use bevy::prelude::*;

use crate::game::player::{
    Activity,
    ActivityTimer,
    JumpCounter,
    Hype,
    Combo,
    InputInfluence,
    Enemy,
    Player,
    Velocity,
};

#[derive(Component, Debug)]
pub enum Behavior {
    Chasing,
    Evading,
    Attacking,
}

pub fn handle_ai_behavior(
    player_query: Query<
        (&Transform, &Hype, &Combo),
        (With<Player>, Without<Enemy>),
    >,
    mut ai_query: Query<
        (&mut Behavior, &Transform, &Hype, &Combo),
        (With<Enemy>, Without<Player>,
    )>,
) {
    if (player_query.get_single().is_err()) {
        return
    }
    let (
        player_transform,
        player_hype,
        player_combo,
    ) = player_query.get_single().unwrap();
    for (
        mut behavior,
        transform,
        hype,
        combo,
    ) in ai_query.iter_mut() {
        let me_to_player = transform.translation - player_transform.translation;
        let me_to_player = Vec2::new(me_to_player.x, me_to_player.y);

        let is_in_range = me_to_player.length() < 100.;
        let my_strength_advantage = hype.0 - player_hype.0 + player_combo.0 - combo.0;
        *behavior = if is_in_range {
            if my_strength_advantage.is_negative() {
                Behavior::Evading
            } else {
                Behavior::Attacking
            }
        } else {
            Behavior::Chasing
        }
    }
}

pub fn handle_ai_flocking(
    mut ai_query: Query<
        (&mut InputInfluence, &Transform, &Velocity, &JumpCounter),
        (With<Enemy>, Without<Player>),
    >
) {
    for (
        mut input_influence,
        transform,
        velocity,
        jumps,
    ) in ai_query.iter_mut() {
        
    }
}

pub fn handle_ai_input(
    player_query: Query<
        &Transform,
        (With<Player>, Without<Enemy>),
    >,
    mut ai_query: Query<
        (&mut InputInfluence, &mut Activity, &Behavior, &Transform, &Velocity, &JumpCounter),
        (With<Enemy>, Without<Player>),
    >,
) {
    if player_query.get_single().is_err() {
        return
    }
    let player_transform = player_query.get_single().unwrap();
    for (
        mut input_influence,
        mut activity,
        mut behavior,
        transform,
        velocity,
        jumps,
    ) in ai_query.iter_mut() {
        let can_execute_action = *activity == Activity::Idle;
        let can_jump = jumps.0 > 0 && match activity.as_ref() {
            Activity::Jump | Activity::Flinch | Activity::Land(_) => false,
            _ => true,
        };
        let me_to_player = transform.translation - player_transform.translation;
        let me_to_player = Vec2::new(me_to_player.x, me_to_player.y);

        match behavior {
            Behavior::Chasing => {
                input_influence.0 = if me_to_player.x.is_sign_positive() { -1. } else { 1. };
                input_influence.1 = if me_to_player.y.is_sign_positive() { -1. } else { 1. };
                if me_to_player.y >= 70. && me_to_player.length() < 150. && can_jump {
                    *activity = Activity::Jump;
                }
            }
            Behavior::Attacking => {
                if can_execute_action {
                    if me_to_player.x.is_sign_positive() && me_to_player.x > me_to_player.y.abs() {
                        *activity = Activity::Counter;
                    } else if me_to_player.x.is_sign_negative() && -me_to_player.x > me_to_player.y.abs() {
                        *activity = Activity::Stomp;
                    } else {
                        *activity = Activity::Jab;
                    }
                }
            }
            Behavior::Evading => {
                input_influence.0 = if me_to_player.x.is_sign_positive() { 1. } else { -1. };
                input_influence.1 = if me_to_player.y.is_sign_positive() { 1. } else { -1. };
                if me_to_player.y <= 30. && me_to_player.length() < 80. && can_jump {
                    *activity = Activity::Jump;
                }
            }
        }
    }
}

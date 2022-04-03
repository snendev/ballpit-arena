use bevy::prelude::*;

#[derive(Debug)]
pub enum ActionType {
    Jump,
    Jab,
    Stomp,
    Counter,
}

#[derive(Debug)]
pub enum Trigger {
    PlayerAction(ActionType),
    PlayerMovement(f32, f32),
    Pause,
}

fn bool_to_num(foo: bool) -> f32 {
    if foo { 1. } else { 0. }
}

#[derive(Debug)]
pub struct Event(pub Trigger);

pub fn handle_keyboard_input(
	mut writer: EventWriter<Event>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        writer.send(Event(Trigger::Pause));
    }
    if keys.just_pressed(KeyCode::Space) {
        writer.send(Event(Trigger::PlayerAction(ActionType::Jump)));
    }
    if keys.just_pressed(KeyCode::R) {
        writer.send(Event(Trigger::PlayerAction(ActionType::Counter)));
    }
    if keys.just_pressed(KeyCode::Q) {
        writer.send(Event(Trigger::PlayerAction(ActionType::Jab)));
    }
    if keys.just_pressed(KeyCode::E) {
        writer.send(Event(Trigger::PlayerAction(ActionType::Stomp)));
    }
    let x = bool_to_num(keys.pressed(KeyCode::D)) - bool_to_num(keys.just_pressed(KeyCode::A));
    let y = bool_to_num(keys.just_pressed(KeyCode::W)) - bool_to_num(keys.just_pressed(KeyCode::S));
    if x != 0. && y != 0. {
        writer.send(Event(Trigger::PlayerMovement(x, y)))
    }
}

pub fn handle_gamepad_input(
	mut writer: EventWriter<Event>,
	gamepads: Res<Gamepads>,
	button_inputs: Res<Input<GamepadButton>>,
	button_axes: Res<Axis<GamepadButton>>,
	axes: Res<Axis<GamepadAxis>>,
    keys: Res<Input<KeyCode>>,
) {
    for gamepad in gamepads.iter().cloned() {
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::Start)) {
            writer.send(Event(Trigger::Pause));
        }

        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::South)) {
            writer.send(Event(Trigger::PlayerAction(ActionType::Jump)));
        }
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::North)) {
            writer.send(Event(Trigger::PlayerAction(ActionType::Counter)));

        }
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::West)) {
            writer.send(Event(Trigger::PlayerAction(ActionType::Jab)));
        }
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::East)) {
            writer.send(Event(Trigger::PlayerAction(ActionType::Stomp)));
        }

        let movement_input_x = axes.get(
            GamepadAxis(gamepad, GamepadAxisType::LeftStickX)
        );
        let movement_input_y = axes.get(
            GamepadAxis(gamepad, GamepadAxisType::LeftStickY)
        );
        let movement_input_vec = if movement_input_x.is_none() && movement_input_y.is_none() {
            None
        } else {
            Some((
                movement_input_x.unwrap_or(0.),
                movement_input_y.unwrap_or(0.),
            ))
        };
        if let Some((x, y)) = movement_input_vec {
            writer.send(Event(Trigger::PlayerMovement(x, y)))
        }
    }
}

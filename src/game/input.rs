use bevy::prelude::*;

#[derive(Debug)]
pub enum ActionType {
    Jump,
    Jab,
    Spike,
    Counter,
}

#[derive(Debug)]
pub enum InputEventType {
    PlayerAction(ActionType),
    Pause,
}

#[derive(Debug)]
pub struct InputEvent(pub InputEventType);

pub fn handle_input(
	mut writer: EventWriter<InputEvent>,
	gamepads: Res<Gamepads>,
	button_inputs: Res<Input<GamepadButton>>,
	button_axes: Res<Axis<GamepadButton>>,
	axes: Res<Axis<GamepadAxis>>,
) {
    for gamepad in gamepads.iter().cloned() {
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::Start)) {
            writer.send(InputEvent(InputEventType::Pause));
        }

        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::South)) {
            writer.send(InputEvent(InputEventType::PlayerAction(ActionType::Jump)));
        }
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::North)) {
            writer.send(InputEvent(InputEventType::PlayerAction(ActionType::Counter)));

        }
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::West)) {
            writer.send(InputEvent(InputEventType::PlayerAction(ActionType::Jab)));
        }
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::East)) {
            writer.send(InputEvent(InputEventType::PlayerAction(ActionType::Spike)));
        }
    }
}

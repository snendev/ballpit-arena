use bevy::prelude::*;

pub fn destroy_recursive<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
  for camera in query.iter() {
      commands.entity(camera).despawn_recursive();
  }
}

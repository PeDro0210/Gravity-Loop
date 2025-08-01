use bevy::ecs::resource::Resource;

#[derive(Resource, Debug)]
pub struct PlayerCarriedAcceleration(pub f32);

impl Default for PlayerCarriedAcceleration {
    fn default() -> Self {
        PlayerCarriedAcceleration(1.0)
    }
}

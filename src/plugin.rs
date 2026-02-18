use bevy::app::{App, Plugin};
use bevy::ecs::schedule::ScheduleCleanupPolicy;
use bevy::log::warn;
use bevy::prelude::IntoScheduleConfigs;
use bevy::render::{ExtractSchedule, RenderApp};
use bevy_sprite_render::{extract_sprites, SpriteRenderPlugin, SpriteSystems};
use crate::{AnimatedKeyframes, AnimatedSprite};
use crate::fake_bevy_sprite_render_extract::my_extract_sprites_with_anime;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<SpriteRenderPlugin>() {
            warn!("No sprite render plugin added.");
            return;
        }
        app.register_required_components::<AnimatedSprite, AnimatedKeyframes>();
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(
                ExtractSchedule,
                my_extract_sprites_with_anime.in_set(SpriteSystems::ExtractSprites),
            );
        }
    }

    fn cleanup(&self, app: &mut App) {
        if !app.is_plugin_added::<SpriteRenderPlugin>() {
            panic!("No sprite render plugin added.");
        }
        if let Some(mut render_app) = app.get_sub_app_mut(RenderApp) {
            let sz = render_app.remove_systems_in_set(
                ExtractSchedule,
                extract_sprites,
                ScheduleCleanupPolicy::RemoveSystemsOnly,
            );
            let sz = sz.expect("Extract sprites unable to be removed");
            assert_eq!(sz, 1, "Expected sprites to be 1, got {}", sz);
        }
    }
}

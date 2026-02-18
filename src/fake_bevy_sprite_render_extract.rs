use bevy::asset::Assets;
use bevy::image::TextureAtlasLayout;
use bevy::math::Rect;
use bevy::prelude::{
    Entity, GlobalTransform, Query, Res, ResMut, Sprite, TextureAtlas, ViewVisibility,
};
use bevy::render::sync_world::RenderEntity;
use bevy::render::Extract;
use bevy::sprite::Anchor;
use bevy::time::Time;
use bevy_sprite_render::{ExtractedSlices, ExtractedSprite, ExtractedSpriteKind, ExtractedSprites};
use crate::{AnimatedKeyframes, AnimatedSprite};
use crate::computed_slices::ComputedTextureSlices;

pub fn my_extract_sprites_with_anime(
    time: Res<Time>,
    mut extracted_sprites: ResMut<ExtractedSprites>,
    mut extracted_slices: ResMut<ExtractedSlices>,
    texture_atlases: Extract<Res<Assets<TextureAtlasLayout>>>,
    sprite_query: Extract<
        Query<(
            Entity,
            RenderEntity,
            &ViewVisibility,
            &Sprite,
            &GlobalTransform,
            &Anchor,
            Option<&AnimatedSprite>,
            Option<&ComputedTextureSlices>,
        )>,
    >,
    animation_query: Extract<Query<&AnimatedKeyframes>>,
) {
    let now_ms = time.elapsed_secs() * 1000.0;
    extracted_sprites.sprites.clear();
    extracted_slices.slices.clear();
    for (
        main_entity,
        render_entity,
        view_visibility,
        sprite,
        transform,
        anchor,
        anim_ref,
        slices,
    ) in sprite_query.iter()
    {
        if !view_visibility.get() {
            continue;
        }

        if let Some(slices) = slices {
            let start = extracted_slices.slices.len();
            extracted_slices
                .slices
                .extend(slices.extract_slices(sprite, anchor.as_vec()));
            let end = extracted_slices.slices.len();
            extracted_sprites.sprites.push(ExtractedSprite {
                main_entity,
                render_entity,
                color: sprite.color.into(),
                transform: *transform,
                flip_x: sprite.flip_x,
                flip_y: sprite.flip_y,
                image_handle_id: sprite.image.id(),
                kind: ExtractedSpriteKind::Slices {
                    indices: start..end,
                },
            });
        } else {
            // Check if we overwrite animation
            let atlas_rect: Option<Rect> = match (&sprite.texture_atlas, anim_ref) {
                (None, _) => None,
                (Some(texture_atlas), None) => texture_atlas
                    .texture_rect(&texture_atlases)
                    .map(|r| r.as_rect()),
                (_, Some(anim_ref)) => {
                    let animation = animation_query.get(anim_ref.animation_ref).expect("Missing animation of dangling animation reference. This should probably be a soft error or warn instead of panic");
                    // let animation = AnimatedKeyframes { keyframes: vec![] };
                    let index = animation.frame_of_animation(now_ms + anim_ref.offset_ms) as usize;
                    let atlas = sprite.texture_atlas.as_ref().unwrap();
                    TextureAtlas {
                        layout: atlas.layout.clone(),
                        index,
                    }
                    .texture_rect(&texture_atlases)
                    .map(|r| r.as_rect())
                }
            };

            let rect = match (atlas_rect, sprite.rect) {
                (None, None) => None,
                (None, Some(sprite_rect)) => Some(sprite_rect),
                (Some(atlas_rect), None) => Some(atlas_rect),
                (Some(atlas_rect), Some(mut sprite_rect)) => {
                    sprite_rect.min += atlas_rect.min;
                    sprite_rect.max += atlas_rect.min;
                    Some(sprite_rect)
                }
            };

            // PERF: we don't check in this function that the `Image` asset is ready, since it should be in most cases and hashing the handle is expensive
            extracted_sprites.sprites.push(ExtractedSprite {
                main_entity,
                render_entity,
                color: sprite.color.into(),
                transform: *transform,
                flip_x: sprite.flip_x,
                flip_y: sprite.flip_y,
                image_handle_id: sprite.image.id(),
                kind: ExtractedSpriteKind::Single {
                    anchor: anchor.as_vec(),
                    rect,
                    scaling_mode: sprite.image_mode.scale(),
                    // Pass the custom size
                    custom_size: sprite.custom_size,
                },
            });
        }
    }
}

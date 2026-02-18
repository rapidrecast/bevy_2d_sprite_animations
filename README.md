# bevy_2d_sprite_animations

2D sprite animations plugin for Bevy.

Sprite animations involve picking a sequence of frames and describing how long they last.
This plugin does  NOT  cover rigging.

# Installation

```bash
cargo add --git https://github.com/rapidrecast/bevy_2d_sprite_animations.git
```

# Usage

## Declare a sprite animation component
```rust
#[derive(Component)]
struct MyWalkingAnimation;

fn spawn_walk_animation(mut commands: Commands) {
    // This is how you would declare keyframes for your animation.
    // You can see that it is simplified to spread out any duration you want over any
    // tile ranges.
    let keyframes = AnimatedKeyframes::new(vec![
        // The first 15 frames happen in 500ms
        (0..15, Duration::from_millis(500)),
        // The last 15 frames happen in 2s
        (15..30, Duration::from_millis(2000)),
    ]);
    commands.spawn((MyWalkingAnimation, keyframes))
}
```

## Declare the animation on any sprite that has an atlas attached

```rust
fn spawn_animated_person(
    mut commands: Commands,
    // We can easily access named sprite keyframes this way
    // Note, we do not need the actual keyframes attached
    walking_animation: Single<Entity, With<MyWalkingAnimation>>
) {
    // You should have the images loaded, preferably as resources.
    let texture_atlas: TextureAtlas = ...;
    let image_handle: Handle<Image> = ...;
    commands.spawn(
        Sprite::from_atlas_image(image_handle, texture_atlas),
        // This is default, but included for demonstration purposes
        Transform::from_xyz(0.0, 0.0, 0.0),
        AnimatedSprite {
            // If you have several animations, you may want them running at different times
            // By using the offset_ms field, you can control how delayed the animation is.
            offset_ms: 0.0,
            animation_ref: *walking_animation,
        }
    );
}
```

# Design choices

This plugin replaces one of the Bevy Core sprite render systems.
It uses the same code, but adds additional component queries to pick up on potential animations.

The reason for this is to handle animations in a stateless way (without `&mut` borrows) and with a single loop (the sprites are sent to the GPU either way, so we may as well use the loop twice).

# Testing, reliability, and production readiness

Use your own judgment.


use bevy::prelude::{Component, Entity};
use bevy::render::render_asset::RenderAsset;
use bevy::render::sync_world::SyncToRenderWorld;
use bevy::sprite::Sprite;
use std::ops::Range;
use std::time::Duration;

/// Example usage
///
///```rust
/// #[derive(Component)]
/// struct MyWalkingAnimation;
///
/// fn create_entity(mut commands: Commands) {
///
///   let animation_ref = commands.spawn((
///     MyWalkingAnimation,
///     AnimatedFrequency::new(vec![
///         // The first 15 frames happen in 500ms
///         (0..15, Duration::from_millis(500)),
///         // The last 15 frames happen in 2s
///         (15..30, Duration::from_millis(2000)),
///     ])
///     )).id();
///
///   commands.spawn((
///     Sprite {
///         // Set the image and sprite atlas
///     ..},
///     AnimatedSprite { offset_frames: 0, current_frame: 0 },
///     AnimationFrequencyRef(Some(animation_ref)),
///     ));
/// }
/// ```
#[derive(Component)]
#[require(Sprite)]
pub struct AnimatedSprite {
    pub offset_ms: f32,
    pub animation_ref: Entity,
}

#[derive(Component, Default)]
#[require(SyncToRenderWorld)]
pub struct AnimatedKeyframes {
    /// Contains the end timestamp of a frame and the frame number
    /// millis as f32 and frame index
    pub keyframes: Vec<(f32, usize)>,
}

impl AnimatedKeyframes {
    pub fn new(frames: Vec<(Range<usize>, Duration)>) -> Self {
        let mut keyframes: Vec<(f32, usize)> = Vec::with_capacity(Self::capacity_hint(&frames));
        for (frame_range, range_duration) in frames {
            let base_duration = keyframes
                .last()
                .map(|(b, _)| b)
                .cloned()
                .unwrap_or_default();
            Self::split_range_into_frame_durations(
                frame_range,
                &range_duration,
                &base_duration,
                &mut keyframes,
            );
        }
        AnimatedKeyframes { keyframes }
    }

    pub fn new_smooth(frames: usize, duration: Duration) -> Self {
        let frames = vec![(0..frames, duration)];
        Self::new(frames)
    }

    fn capacity_hint(frames: &Vec<(Range<usize>, Duration)>) -> usize {
        let mut total = 0;
        for (r, _) in frames {
            total += r.end - r.start;
        }
        total as usize
    }

    fn split_range_into_frame_durations(
        frames: Range<usize>,
        duration: &Duration,
        base_duration: &f32,
        target: &mut Vec<(f32, usize)>,
    ) {
        let num_frames = frames.end - frames.start;
        let frame_duration_millis = duration.as_secs_f32() * 1000.0 / num_frames as f32;
        target.extend(
            frames
                .into_iter()
                .enumerate()
                .map(|(seq_in_duration_window, frame)| {
                    let end_duration = frame_duration_millis * (seq_in_duration_window + 1) as f32;
                    (base_duration + end_duration, frame)
                }),
        );
    }

    pub fn frame_of_animation(&self, current_millis: f32) -> usize {
        let last_duration = self.keyframes.last().expect("There must be at least 1 frame of animation to check which frame of animation we are in").0;
        let in_frame = current_millis % last_duration;
        for (frame_end, frame_num) in &self.keyframes {
            if in_frame < *frame_end {
                return *frame_num;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use crate::AnimatedKeyframes;

    #[test]
    fn test_animation() {
        let frequency = AnimatedKeyframes::new(vec![
            (0..15, Duration::from_millis(500)),
            (15..29, Duration::from_millis(2000)),
            (29..30, Duration::from_millis(4000)),
        ]);
        assert_eq!(
            frequency.keyframes,
            vec![
                (33.0, 0),
                (66.0, 1),
                (99.0, 2),
                (132.0, 3),
                (165.0, 4),
                (198.0, 5),
                (231.0, 6),
                (264.0, 7),
                (297.0, 8),
                (330.0, 9),
                (363.0, 10),
                (396.0, 11),
                (429.0, 12),
                (462.0, 13),
                (495.0, 14),
                (637.0, 15),
                (779.0, 16),
                (921.0, 17),
                (1063.0, 18),
                (1205.0, 19),
                (1347.0, 20),
                (1489.0, 21),
                (1631.0, 22),
                (1773.0, 23),
                (1915.0, 24),
                (2057.0, 25),
                (2199.0, 26),
                (2341.0, 27),
                (2483.0, 28),
                (6483.0, 29)
            ]
        );
        let test_data = [
            (0f32, 0),
            (33., 1),
            (66., 2),
            (99., 3),
            (4001., 29),
            (6483. - 1.0, 29),
            (6483., 0),
            (6483. + 1.0, 0),
            ((2.0 * 6483.0) - 1.0, 29),
            (2.0 * 6483.0, 0),
            ((2.0 * 6483.0) + 1.0, 0),
        ];
        for (ts, expected_frame) in test_data {
            let actual = frequency.frame_of_animation(ts);
            assert_eq!(
                actual, expected_frame,
                "TS={ts}, expected={expected_frame} but was {actual}"
            );
        }
    }
}

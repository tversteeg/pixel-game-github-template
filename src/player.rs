use crate::sprite;
use anyhow::Result;
use specs_blit::{specs::prelude::*, Sprite};
use sprite_gen::{MaskValue::*, Options};

/// Spawn a new player.
pub fn spawn_player(world: &mut World) -> Result<()> {
    let (width, _height, options) = (
        12,
        20,
        Options {
            mirror_x: true,
            mirror_y: false,
            colored: true,
            edge_brightness: 0.3,
            color_variations: 0.21,
            brightness_noise: 0.31,
            saturation: 0.5,
            seed: 0,
        },
    );
    let data = [
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body1, Empty, Empty,
        Empty, Empty, Empty, Empty, Body1, Body1, Body1, Body1, Body1, Body1, Empty, Empty, Empty,
        Empty, Empty, Body1, Body2, Body2, Body2, Body2, Body1, Body1, Empty, Empty, Empty, Empty,
        Empty, Empty, Body1, Body1, Body1, Body1, Body1, Body1, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Body1, Body1, Body1, Body2, Body1, Empty, Empty, Body1, Empty, Empty, Empty,
        Empty, Body1, Body1, Body1, Body1, Body2, Empty, Empty, Body1, Empty, Empty, Empty, Body1,
        Body1, Body1, Body1, Body1, Body2, Empty, Empty, Body1, Empty, Empty, Body1, Body1, Body1,
        Body1, Body1, Body2, Body2, Empty, Body1, Body1, Empty, Empty, Body1, Body1, Body1, Body1,
        Body1, Body2, Body2, Empty, Body1, Empty, Empty, Empty, Body1, Body1, Body2, Body2, Body1,
        Body2, Body2, Empty, Body1, Empty, Empty, Body2, Body2, Body2, Body2, Body2, Body2, Body2,
        Body2, Empty, Body1, Empty, Empty, Body2, Body2, Body2, Body1, Body2, Body2, Body1, Body2,
        Empty, Body1, Empty, Body2, Body2, Body1, Body2, Body1, Body2, Body2, Body1, Body2, Empty,
        Body1, Body2, Body2, Body1, Body1, Body2, Body1, Body2, Body2, Body1, Body2, Empty, Body1,
        Body2, Empty, Body1, Body1, Body2, Body1, Body2, Body2, Body1, Body2, Empty, Body2, Body2,
        Empty, Body1, Body1, Body2, Body2, Body2, Body1, Body1, Body2, Empty, Body2, Body2, Body2,
        Body2, Body2, Body2, Body2, Body1, Body1, Body1, Body1, Empty, Body1, Body1, Body1, Body1,
        Body1, Body1, Body1, Body1, Body1, Body1, Body1, Empty, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Empty, Empty, Empty, Empty,
    ];

    let sprite = sprite::generate(width, options, &data, 4)?;
    world.create_entity().with(Sprite::new(sprite)).build();

    Ok(())
}

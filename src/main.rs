mod audio;

use crate::audio::Audio;
use anyhow::{anyhow, Result};
use minifb::{Scale, ScaleMode, Window, WindowOptions};
use specs::prelude::*;
use specs_blit::{PixelBuffer, RenderSystem, Sprite};
use std::time::Duration;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() -> Result<()> {
    // Setup the ECS system
    let mut world = World::new();

    // Load the sprite rendering component
    world.register::<Sprite>();
    // Add the pixel buffer as a resource so it can be accessed from the RenderSystem later, to be
    // updated every frame
    world.insert(PixelBuffer::new(WIDTH, HEIGHT));

    // Add the audio system
    world.insert(Audio::new());

    // Setup the dispatcher with the blit system
    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(RenderSystem)
        .build();

    // Setup minifb window related things
    let title = format!("replace_me {}", env!("CARGO_PKG_VERSION"));
    let options = WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        topmost: false,
        scale: Scale::X2,
        scale_mode: ScaleMode::AspectRatioStretch,
    };
    let mut window = Window::new(&title, WIDTH, HEIGHT, options).expect("Unable to open window");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    {
        // Start the audio
        let mut audio = world.write_resource::<Audio>();
        audio.run();
    }

    while window.is_open() {
        // Clear the buffer with a black color
        {
            let mut buffer = world.write_resource::<PixelBuffer>();
            buffer.clear(0);
        }

        // Update specs
        dispatcher.dispatch(&world);

        // Add/remove entities added in dispatch through `LazyUpdate`
        world.maintain();

        // Get the pixel buffer to render it
        let buffer = world.read_resource::<PixelBuffer>();
        // Render the pixel buffer
        window
            .update_with_buffer(&buffer.pixels(), buffer.width(), buffer.height())
            .map_err(|e| anyhow!("Could not update pixel buffer: {}", e))?;
    }

    Ok(())
}

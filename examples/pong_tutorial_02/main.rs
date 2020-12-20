//! Pong Tutorial 2

mod pong;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        rendy::hal::command::ClearColor,
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("examples/pong_tutorial_02/config/display.ron");

    // This line is not mentioned in the pong tutorial as it is specific to the context
    // of the git repository. It only is a different location to load the assets from.
    let assets_dir = app_root.join("examples/pong_tutorial_02/assets/");

    let mut dispatcher = DispatcherBuilder::default();
    dispatcher.add_bundle(TransformBundle).add_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and
            // drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?.with_clear(ClearColor {
                    float32: [0.0, 0.0, 0.0, 1.0],
                }),
            )
            // RenderFlat2D plugin is used to render entities with `SpriteRender` component.
            .with_plugin(RenderFlat2D::default()),
    );

    let mut game = Application::new(assets_dir, Pong, dispatcher)?;
    game.run();
    Ok(())
}

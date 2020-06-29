use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    utils::{
        application_root_dir,
        fps_counter::FpsCounterBundle,
    },
};

mod states;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        // .with_system_desc(UiEventHandlerSystemDesc::default(), "ui_event_handler", &[])
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(amethyst::ui::RenderUi::default()),
        )?
        .with_bundle(amethyst::ui::UiBundle::<StringBindings>::new())?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, states::main_menu::MenuPrincipal, game_data)?;
    game.run();

    Ok(())
}

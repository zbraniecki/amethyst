use amethyst::{
    assets::{PrefabLoader, PrefabLoaderSystem, RonFormat},
    core::transform::TransformBundle,
    ecs::{World, WorldExt},
    prelude::*,
    renderer::{
        plugins::{RenderPbr3D, RenderToWindow},
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::{application_root_dir, scene::BasicScenePrefab},
};

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>;

struct Example;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let handle = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/spotlights_scene.ron", RonFormat, ())
        });
        data.world.create_entity().with(handle).build();
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("examples/spotlights/config/display.ron");
    let assets_dir = app_root.join("examples/assets/");

    let mut world = World::with_application_resources::<GameData<'_, '_>, _>(assets_dir)?;

    let game_data = GameDataBuilder::default()
        .with(PrefabLoaderSystem::<MyPrefabData>::new(&mut world), "", &[])
        .with_bundle(&mut world, TransformBundle::new())?
        .with_bundle(
            &mut world,
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderPbr3D::default()),
        )?;
    let mut game = Application::new(Example, game_data, world)?;
    game.run();
    Ok(())
}

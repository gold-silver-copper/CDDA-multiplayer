pub use rlg::*;

fn main() {
    App::new()
        .init_resource::<Cli>() // Parse CLI before creating window.
        .init_resource::<ItemSpawnQueue>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            ReplicationPlugins,
            CCCPlugin,
        )) // prevents blurry sprites
        .add_plugins(EguiPlugin)
        .run();
}

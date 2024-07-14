use bevy::{
    app::{App, Startup},
    diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin},
    prelude::{Camera2dBundle, Commands},
    DefaultPlugins,
};
use iyes_perf_ui::{entries::PerfUiCompleteBundle, PerfUiPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            PerfUiPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(PerfUiCompleteBundle::default());
}

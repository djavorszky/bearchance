use bevy::core::FixedTimestep;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let fps_counter_stage = SystemStage::parallel()
            .with_run_criteria(FixedTimestep::steps_per_second(1.0))
            .with_system(fps.system());

        app.add_system(bevy::input::system::exit_on_esc_system.system())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_stage_after(stage::UPDATE, "FPS counter stage", fps_counter_stage);
    }
}


fn fps(diagnostics: Res<Diagnostics>) {
    let fps = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).unwrap();

    println!("Current FPS: {:.1}, average: {:.1}", fps.value().unwrap(), fps.average().unwrap());
}

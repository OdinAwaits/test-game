use bevy::prelude::*;
use iyes_perf_ui::prelude::*;
use bevy_butler::*;


#[butler_plugin_group]
pub struct All;

#[butler_plugin]
#[add_plugin(to_group= All)]
pub struct Debug;

#[add_plugin(to_plugin= Debug)]
use iyes_perf_ui::PerfUiPlugin;
#[add_plugin(to_plugin= Debug)]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
#[add_plugin(to_plugin= Debug)]
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
#[add_plugin(to_plugin= Debug)]
use bevy::render::diagnostic::RenderDiagnosticsPlugin;

#[derive(Component)]
#[require(
        PerfUiRoot = PerfUiRoot{
        fontsize_label: 24.0,
        fontsize_value: 24.0,
        ..default()
    },
    PerfUiEntryFPS,
    PerfUiEntryFPSAverage,
    PerfUiEntryFPSPctLow,
    PerfUiEntryFPSWorst,
    PerfUiEntryFrameTime,
    PerfUiEntryFrameTimeWorst,
    PerfUiEntryFrameCount,
    PerfUiEntryEntityCount,
    PerfUiEntryFixedTimeStep,
    PerfUiEntryFixedOverstep,
    PerfUiEntryRunningTime,
    PerfUiEntryCursorPosition,
    PerfUiEntryWindowResolution,
    PerfUiEntryWindowScaleFactor,
    PerfUiEntryWindowMode,
    PerfUiEntryWindowPresentMode,
)]
struct PerfStatUI;
#[add_system(
    plugin= Debug,
    schedule= Startup
)]
fn startup (mut commands: Commands) {
    commands.spawn(PerfStatUI);
}





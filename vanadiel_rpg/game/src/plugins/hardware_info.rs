//! Simple plugin to log CPU and GPU information on startup.

use bevy::prelude::*;
use bevy::render::renderer::RenderAdapterInfo;
use sysinfo::{CpuExt, System, SystemExt};

/// Plugin that prints basic hardware information.
pub struct HardwareInfoPlugin;

impl Plugin for HardwareInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, log_hardware_info);
    }
}

fn log_hardware_info(adapter: Res<RenderAdapterInfo>) {
    let mut system = System::new();
    system.refresh_cpu();
    let cpu_brand = system
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string());

    info!("Detected GPU: {}", adapter.name);
    info!("Detected CPU: {}", cpu_brand);
}

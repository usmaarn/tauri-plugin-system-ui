use serde::{Deserialize, Serialize};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;

#[cfg(mobile)]
mod mobile;

mod commands;
mod error;

pub use error::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemBarsConfig {
    pub status_bar: Option<BarConfig>,
    pub navigation_bar: Option<BarConfig>,
    pub edge_to_edge: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BarConfig {
    pub background_color: Option<String>,
    pub icon_style: Option<IconStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IconStyle {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SystemBarTheme {
    Light,
    Dark,
    System,
}

impl SystemBarTheme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::System => "system",
        }
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("system-ui")
        .invoke_handler(tauri::generate_handler![
            commands::configure_system_bars,
            commands::set_system_bar_theme,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let plugin = mobile::init(app, api)?;

            #[cfg(desktop)]
            let plugin = desktop::init(app, api)?;

            app.manage(plugin);
            Ok(())
        })
        .build()
}

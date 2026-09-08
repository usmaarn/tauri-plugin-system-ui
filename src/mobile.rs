use crate::{Error, Result, SystemBarTheme, SystemBarsConfig};
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_system_ui);

pub struct SystemUi<R: Runtime>(PluginHandle<R>);

pub fn init<R: Runtime, C: serde::de::DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> Result<SystemUi<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(
        "com.tauri.systemui",
        "SystemUiPlugin",
    )?;

    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_system_ui)?;

    Ok(SystemUi(handle))
}

impl<R: Runtime> SystemUi<R> {
    pub fn configure(&self, config: SystemBarsConfig) -> Result<()> {
        self.0
            .run_mobile_plugin("configureSystemBars", config)
            .map(|_: serde_json::Value| ())
            .map_err(Error::from)
    }

    pub fn set_theme(&self, theme: SystemBarTheme) -> Result<()> {
        self.0
            .run_mobile_plugin(
                "setSystemBarTheme",
                serde_json::json!({ "theme": theme.as_str() }),
            )
            .map(|_: serde_json::Value| ())
            .map_err(Error::from)
    }
}

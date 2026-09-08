use crate::{Result, SystemBarTheme, SystemBarsConfig};
use tauri::{AppHandle, Runtime};

pub struct SystemUi<R: Runtime> {
    _marker: std::marker::PhantomData<R>,
}

pub fn init<R: Runtime, C>(
    _app: &AppHandle<R>,
    _api: tauri::plugin::PluginApi<R, C>,
) -> Result<SystemUi<R>>
where
    C: serde::de::DeserializeOwned,
{
    Ok(SystemUi {
        _marker: std::marker::PhantomData,
    })
}

impl<R: Runtime> SystemUi<R> {
    pub fn configure(&self, _config: SystemBarsConfig) -> Result<()> {
        Ok(())
    }

    pub fn set_theme(&self, _theme: SystemBarTheme) -> Result<()> {
        Ok(())
    }
}

use crate::{SystemBarTheme, SystemBarsConfig};
use tauri::{command, AppHandle, Runtime};

#[command]
pub fn configure_system_bars<R: Runtime>(
    app: AppHandle<R>,
    config: SystemBarsConfig,
) -> Result<(), String> {
    #[cfg(mobile)]
    {
        let plugin = app
            .try_state::<crate::mobile::SystemUi<R>>()
            .ok_or_else(|| "system-ui plugin is not initialized".to_string())?;

        plugin
            .configure_system_bars(config)
            .map_err(|e| e.to_string())?;
    }

    #[cfg(desktop)]
    {
        let _ = app;
        let _ = config;
    }

    Ok(())
}

#[command]
pub fn set_system_bar_theme<R: Runtime>(
    app: AppHandle<R>,
    theme: SystemBarTheme,
) -> Result<(), String> {
    #[cfg(mobile)]
    {
        let plugin = app
            .try_state::<crate::mobile::SystemUi<R>>()
            .ok_or_else(|| "system-ui plugin is not initialized".to_string())?;

        plugin
            .set_system_bar_theme(theme)
            .map_err(|e| e.to_string())?;
    }

    #[cfg(desktop)]
    {
        let _ = app;
        let _ = theme;
    }

    Ok(())
}

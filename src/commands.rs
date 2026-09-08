use crate::{mobile::SystemUi, Result, SystemBarTheme, SystemBarsConfig};
use tauri::{command, Runtime, State};

#[command]
pub fn configure_system_bars<R: Runtime>(
    state: State<'_, SystemUi<R>>,
    config: SystemBarsConfig,
) -> Result<()> {
    state.configure(config)
}

#[command]
pub fn set_system_bar_theme<R: Runtime>(
    state: State<'_, SystemUi<R>>,
    theme: SystemBarTheme,
) -> Result<()> {
    state.set_theme(theme)
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("Mobile plugin error: {0}")]
    MobilePlugin(#[from] tauri::plugin::mobile::PluginInvokeError),
}

pub type Result<T> = std::result::Result<T, Error>;

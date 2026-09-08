import { invoke } from '@tauri-apps/api/core';

function configureSystemBars(config) {
    return invoke("plugin:system-ui|configure_system_bars", {
        config,
    });
}
function setSystemBarTheme(theme) {
    return invoke("plugin:system-ui|set_system_bar_theme", {
        theme,
    });
}

export { configureSystemBars, setSystemBarTheme };

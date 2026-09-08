import { invoke } from "@tauri-apps/api/core";

export type IconStyle = "light" | "dark" | "system";
export type SystemBarTheme = "light" | "dark" | "system";

export interface BarConfig {
  backgroundColor?: string;
  iconStyle?: IconStyle;
}

export interface SystemBarsConfig {
  statusBar?: BarConfig;
  navigationBar?: BarConfig;
  edgeToEdge?: boolean;
}

export function configureSystemBars(
  config: SystemBarsConfig,
): Promise<void> {
  return invoke("plugin:system-ui|configure_system_bars", {
    config,
  });
}

export function setSystemBarTheme(
  theme: SystemBarTheme,
): Promise<void> {
  return invoke("plugin:system-ui|set_system_bar_theme", {
    theme,
  });
}

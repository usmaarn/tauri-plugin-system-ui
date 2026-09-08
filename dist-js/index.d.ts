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
export declare function configureSystemBars(config: SystemBarsConfig): Promise<void>;
export declare function setSystemBarTheme(theme: SystemBarTheme): Promise<void>;

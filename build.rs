fn main() {
    tauri_plugin::Builder::new(&[
        "configure_system_bars",
        "set_system_bar_theme",
    ])
    .build();
}

# tauri-plugin-system-ui

Reusable Tauri 2 mobile plugin for configuring Android system bars and iOS status-bar appearance.

## Features

- Android status-bar background color
- Android status-bar icon appearance
- Android navigation-bar background color
- Android navigation-bar icon appearance
- Android edge-to-edge toggle
- iOS status-bar appearance
- `light`, `dark`, and `system` themes
- JS/TypeScript API
- Safe-area CSS guidance

Tauri mobile plugins support native Kotlin and Swift implementations. See the official Tauri mobile plugin documentation:
https://v2.tauri.app/develop/plugins/develop-mobile/

## API

```ts
import { configureSystemBars, setSystemBarTheme } from "tauri-plugin-system-ui";

await setSystemBarTheme("dark");

await configureSystemBars({
  statusBar: {
    backgroundColor: "#111827",
    iconStyle: "light",
  },
  navigationBar: {
    backgroundColor: "#111827",
    iconStyle: "light",
  },
  edgeToEdge: true,
});
```

On iOS, `navigationBar` is intentionally ignored. iOS does not expose an Android-style configurable system navigation-bar background for a Tauri WebView. Use `env(safe-area-inset-bottom)` and your web UI background instead.

## Installation

After publishing:

```bash
npm install tauri-plugin-system-ui
```

In `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-plugin-system-ui = "0.1"
```

Register it:

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_system_ui::init())
```

Add the permission to the capability that should access the plugin:

```json
{
  "permissions": [
    "system-ui:default"
  ]
}
```

## Safe-area CSS

```css
:root {
  --safe-top: env(safe-area-inset-top, 0px);
  --safe-right: env(safe-area-inset-right, 0px);
  --safe-bottom: env(safe-area-inset-bottom, 0px);
  --safe-left: env(safe-area-inset-left, 0px);
}

.app {
  padding-top: var(--safe-top);
  padding-bottom: var(--safe-bottom);
}
```

For an edge-to-edge layout, prefer transparent system bars and let the web UI paint behind them.

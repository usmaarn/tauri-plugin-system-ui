# Install in an existing Tauri 2 app

## Recommended

1. Generate a fresh plugin scaffold with your installed Tauri CLI:

```bash
npx @tauri-apps/cli plugin new system-ui --android --ios
```

2. Copy this repository's `src`, `guest-js`, `permissions`, `build.rs`, `Cargo.toml`, `package.json`, and the Android/iOS source files over the generated scaffold.

3. In `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-plugin-system-ui = { path = "plugins/system-ui" }
```

4. Register:

```rust
.plugin(tauri_plugin_system_ui::init())
```

5. Add to the app capability:

```json
"system-ui:default"
```

6. Build the JS package and install/link it from the frontend.

For a local monorepo:

```bash
npm install ./src-tauri/plugins/system-ui
```

Then:

```ts
import {
  configureSystemBars,
  setSystemBarTheme,
} from "tauri-plugin-system-ui";

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

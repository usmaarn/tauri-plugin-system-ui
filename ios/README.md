The Swift package source is included here.

For the first release, generate the iOS plugin project with:
npx @tauri-apps/cli plugin new system-ui --android --ios

Then replace the generated Package.swift and Sources/SystemUiPlugin.swift with the files in this repository.

Tauri's CLI owns generated mobile build scaffolding; keeping that scaffolding generated avoids version-specific breakage.

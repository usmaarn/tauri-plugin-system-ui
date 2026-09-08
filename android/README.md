The Kotlin source is included here.

For the first release, generate the Android plugin project with:
npx @tauri-apps/cli plugin new system-ui --android --ios

Then replace the generated Kotlin source with:
android/src/main/java/com/tauri/systemui/SystemUiPlugin.kt

This keeps Gradle/template files aligned with your installed Tauri CLI version.

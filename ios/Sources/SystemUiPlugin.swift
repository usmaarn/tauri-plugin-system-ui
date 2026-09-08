import Foundation
import SwiftRs
import Tauri
import UIKit
import WebKit

private struct BarConfig: Decodable {
    let backgroundColor: String?
    let iconStyle: String?
}

private struct SystemBarsConfig: Decodable {
    let statusBar: BarConfig?
    let navigationBar: BarConfig?
    let edgeToEdge: Bool?
}

private struct ThemeArgs: Decodable {
    let theme: String
}

final class SystemUiPlugin: Plugin {

    @objc public func configureSystemBars(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(SystemBarsConfig.self)

        DispatchQueue.main.async {
            self.apply(config: args)
        }

        invoke.resolve()
    }

    @objc public func setSystemBarTheme(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(ThemeArgs.self)

        DispatchQueue.main.async {
            self.applyTheme(args.theme)
        }

        invoke.resolve()
    }

    private func apply(config: SystemBarsConfig) {
        guard let window = activeWindow() else { return }

        if let edgeToEdge = config.edgeToEdge {
            if edgeToEdge {
                window.isOpaque = false
                window.backgroundColor = .clear
            }
        }

        if let status = config.statusBar,
           let style = status.iconStyle {
            applyStatusBarStyle(style, window: window)
        }
    }

    private func applyTheme(_ theme: String) {
        guard let window = activeWindow() else { return }

        switch theme {
        case "light":
            window.overrideUserInterfaceStyle = .light
        case "dark":
            window.overrideUserInterfaceStyle = .dark
        case "system":
            window.overrideUserInterfaceStyle = .unspecified
        default:
            return
        }

        window.rootViewController?.setNeedsStatusBarAppearanceUpdate()
    }

    private func applyStatusBarStyle(_ style: String, window: UIWindow) {
        switch style {
        case "light":
            window.overrideUserInterfaceStyle = .dark
        case "dark":
            window.overrideUserInterfaceStyle = .light
        case "system":
            window.overrideUserInterfaceStyle = .unspecified
        default:
            break
        }

        window.rootViewController?.setNeedsStatusBarAppearanceUpdate()
    }

    private func activeWindow() -> UIWindow? {
        let scenes = UIApplication.shared.connectedScenes

        for scene in scenes {
            guard let windowScene = scene as? UIWindowScene else {
                continue
            }

            guard scene.activationState == .foregroundActive ||
                  scene.activationState == .foregroundInactive else {
                continue
            }

            if let keyWindow = windowScene.windows.first(where: { $0.isKeyWindow }) {
                return keyWindow
            }

            if let window = windowScene.windows.first {
                return window
            }
        }

        return nil
    }
}

@_cdecl("init_plugin_system_ui")
func initPlugin(name: SRString, webview: WKWebView?) {
    Tauri.registerPlugin(
        webview: webview,
        name: name.toString(),
        plugin: SystemUiPlugin()
    )
}

package com.tauri.systemui

import android.app.Activity
import android.content.res.Configuration
import android.graphics.Color
import android.os.Build
import android.view.View
import android.view.WindowInsetsController

import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin

@InvokeArg
class BarConfig {
    var backgroundColor: String? = null
    var iconStyle: String? = null
}

@InvokeArg
class SystemBarsConfig {
    var statusBar: BarConfig? = null
    var navigationBar: BarConfig? = null
    var edgeToEdge: Boolean? = null
}

@InvokeArg
class ThemeArgs {
    lateinit var theme: String
}

@TauriPlugin
class SystemUiPlugin(private val activity: Activity) : Plugin(activity) {

    @Command
    fun configureSystemBars(invoke: Invoke) {
        try {
            val args = invoke.parseArgs(SystemBarsConfig::class.java)

            activity.runOnUiThread {
                args.edgeToEdge?.let { setEdgeToEdge(it) }
                args.statusBar?.let { configureStatusBar(it) }
                args.navigationBar?.let { configureNavigationBar(it) }
            }

            invoke.resolve()
        } catch (e: Exception) {
            invoke.reject(e.message ?: "Failed to configure system bars")
        }
    }

    @Command
    fun setSystemBarTheme(invoke: Invoke) {
        try {
            val args = invoke.parseArgs(ThemeArgs::class.java)

            activity.runOnUiThread {
                when (args.theme) {
                    "light" -> {
                        setStatusBarIcons(darkIcons = true)
                        setNavigationBarIcons(darkIcons = true)
                    }
                    "dark" -> {
                        setStatusBarIcons(darkIcons = false)
                        setNavigationBarIcons(darkIcons = false)
                    }
                    "system" -> {
                        val nightMode = activity.resources.configuration.uiMode and
                            Configuration.UI_MODE_NIGHT_MASK

                        val dark = nightMode == Configuration.UI_MODE_NIGHT_YES

                        setStatusBarIcons(darkIcons = !dark)
                        setNavigationBarIcons(darkIcons = !dark)
                    }
                    else -> throw IllegalArgumentException(
                        "theme must be light, dark, or system"
                    )
                }
            }

            invoke.resolve()
        } catch (e: Exception) {
            invoke.reject(e.message ?: "Failed to set system bar theme")
        }
    }

    private fun configureStatusBar(config: BarConfig) {
        config.backgroundColor?.let {
            activity.window.statusBarColor = parseColor(it)
        }

        config.iconStyle?.let {
            when (it) {
                "light" -> setStatusBarIcons(false)
                "dark" -> setStatusBarIcons(true)
                "system" -> setStatusBarIconsFromSystem()
            }
        }
    }

    private fun configureNavigationBar(config: BarConfig) {
        config.backgroundColor?.let {
            activity.window.navigationBarColor = parseColor(it)
        }

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            activity.window.isNavigationBarContrastEnforced = false
        }

        config.iconStyle?.let {
            when (it) {
                "light" -> setNavigationBarIcons(false)
                "dark" -> setNavigationBarIcons(true)
                "system" -> setNavigationBarIconsFromSystem()
            }
        }
    }

    private fun setStatusBarIcons(darkIcons: Boolean) {
        val window = activity.window

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            val controller = window.insetsController ?: return
            val appearance =
                if (darkIcons) WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS else 0

            controller.setSystemBarsAppearance(
                appearance,
                WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS
            )
        } else {
            @Suppress("DEPRECATION")
            var flags = window.decorView.systemUiVisibility

            @Suppress("DEPRECATION")
            val lightFlag = View.SYSTEM_UI_FLAG_LIGHT_STATUS_BAR

            flags = if (darkIcons) {
                flags or lightFlag
            } else {
                flags and lightFlag.inv()
            }

            @Suppress("DEPRECATION")
            window.decorView.systemUiVisibility = flags
        }
    }

    private fun setNavigationBarIcons(darkIcons: Boolean) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            val controller = activity.window.insetsController ?: return
            val appearance =
                if (darkIcons) WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS else 0

            controller.setSystemBarsAppearance(
                appearance,
                WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS
            )
        } else if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            @Suppress("DEPRECATION")
            var flags = activity.window.decorView.systemUiVisibility

            @Suppress("DEPRECATION")
            val lightFlag = View.SYSTEM_UI_FLAG_LIGHT_NAVIGATION_BAR

            flags = if (darkIcons) {
                flags or lightFlag
            } else {
                flags and lightFlag.inv()
            }

            @Suppress("DEPRECATION")
            activity.window.decorView.systemUiVisibility = flags
        }
    }

    private fun setStatusBarIconsFromSystem() {
        val dark = isSystemDark()
        setStatusBarIcons(darkIcons = !dark)
    }

    private fun setNavigationBarIconsFromSystem() {
        val dark = isSystemDark()
        setNavigationBarIcons(darkIcons = !dark)
    }

    private fun setEdgeToEdge(enabled: Boolean) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            activity.window.setDecorFitsSystemWindows(!enabled)

            if (enabled) {
                activity.window.statusBarColor = Color.TRANSPARENT
                activity.window.navigationBarColor = Color.TRANSPARENT
            }
        }
    }

    private fun isSystemDark(): Boolean {
        val mode = activity.resources.configuration.uiMode and
            Configuration.UI_MODE_NIGHT_MASK

        return mode == Configuration.UI_MODE_NIGHT_YES
    }

    private fun parseColor(value: String): Int {
        return Color.parseColor(value)
    }
}

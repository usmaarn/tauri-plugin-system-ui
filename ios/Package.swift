// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "tauri-plugin-system-ui",
    platforms: [
        .iOS(.v13)
    ],
    products: [
        .library(
            name: "tauri-plugin-system-ui",
            type: .static,
            targets: ["tauri-plugin-system-ui"]
        )
    ],
    dependencies: [
        .package(name: "Tauri", path: "../.tauri/tauri-api"),
        .package(name: "SwiftRs", url: "https://github.com/Brendonovich/swift-rs", from: "1.0.0")
    ],
    targets: [
        .target(
            name: "tauri-plugin-system-ui",
            dependencies: [
                .byName(name: "Tauri"),
                .product(name: "SwiftRs", package: "SwiftRs")
            ],
            path: "Sources"
        )
    ]
)

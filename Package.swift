// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "TreeSitterRust",
    products: [
        .library(name: "TreeSitterRust", targets: ["TreeSitterRust"]),
    ],
    dependencies: [
        .package(url: "https://github.com/tree-sitter/swift-tree-sitter", from: "0.9.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterRust",
            dependencies: [],
            path: ".",
            sources: [
                "src/parser.c",
                "src/scanner.c",
            ],
            resources: [
                .copy("queries")
            ],
            publicHeadersPath: "bindings/swift",
            cSettings: [.headerSearchPath("src")]
        ),
        .testTarget(
            name: "TreeSitterRustTests",
            dependencies: [
                .product(name: "SwiftTreeSitter", package: "swift-tree-sitter"),
                "TreeSitterRust",
            ],
            path: "bindings/swift/TreeSitterRustTests"
        )
    ],
    cLanguageStandard: .c11
)

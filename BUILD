load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")

rust_binary(
    name = "hello_world",
    srcs = ["src/main.rs"],
    deps = all_crate_deps(normal = True,),
)

load("@rules_rust//crate_universe:defs.bzl", "crates_vendor")

crates_vendor(
    name = "crates_vendor",
    manifests = [":Cargo.toml"],
    mode = "local",
    vendor_path = ":crates",
)

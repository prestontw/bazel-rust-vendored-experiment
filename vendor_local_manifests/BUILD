load("@rules_rust//crate_universe:defs.bzl", "crates_vendor")

crates_vendor(
    name = "crates_vendor",
    manifests = [":Cargo.toml"],
    mode = "local",
    vendor_path = "crates",
)

load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")
load("//vendor_local_manifests/crates:defs.bzl", "aliases", "all_crate_deps")

rust_binary(
    name = "hello_world",
    srcs = ["src/main.rs"],
    deps = all_crate_deps(normal = True,),
    aliases = aliases(),
)

rust_test(
    name = "simple",
    srcs = ["src/main.rs"],
    deps = all_crate_deps(normal = True,),
    aliases = aliases(),
)

load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")

rust_binary(
    name = "hello_world",
    srcs = ["src/main.rs"],
    deps = all_crate_deps(normal = True,),
)

### Sample app for testing rust and bazel
I'm trying to set up a locally vendored rust project that supports building
with both Cargo and bazel.

This is mostly a cargo project with bazel thrown in later. The errors I encountered
made me borrow the structure of https://github.com/bazelbuild/rules_rust/tree/main/examples/crate_universe/vendor_local_manifests.

#### Does it work?

Yes! For running from cargo:
```sh
# in `vendor_local_manifests`
cargo run
```

For testing:
```sh
# in `vendor_local_manifests`
cargo test
```

For running from bazel:
```sh
bazel run //vendor_local_manifests:hello_world
```

For testing:
```
bazel test //...
```
to test everything.

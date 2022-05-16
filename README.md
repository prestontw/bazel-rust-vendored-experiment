### Sample app for testing rust and bazel
I'm trying to set up a locally vendored rust project that supports building
with both Cargo and bazel.

#### Does it work?

Almost! Running
```
bazel run //:crates_vendor
```

I see
```
Error: Failed to parse label from string: @__main__///:crates/regex-syntax-0.6.25:BUILD.bazel
```
(or a bunch of very similar errors for different crates).

But if you just want to run the project through bazel, not caring about vendoring,
```
bazel run //:hello_world
```
works! Test by navigating to `http://localhost:3000/` after running the above command.

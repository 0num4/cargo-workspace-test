cargo add wgpu をする

```
user: root …/work/private/test/cargo-workspace-test/learning-wgpu on  main [!] is 📦 v0.1.0 via 🦀 v1.80.0
❯ cargo add wgpu
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
    Updating crates.io index
      Adding wgpu v22.1.0 to dependencies
             Features:
             + dx12
             + metal
             + webgpu
             + wgsl
             - angle
             - counters
             - fragile-send-sync-non-atomic-wasm
             - glsl
             - naga-ir
             - replay
             - serde
             - spirv
             - strict_asserts
             - vulkan-portability
             - webgl
             - wgc
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
    Updating crates.io index
     Locking 114 packages to latest compatible versions
      Adding bit-set v0.6.0 (latest: v0.8.0)
      Adding bit-vec v0.7.0 (latest: v0.8.0)
      Adding bitflags v1.3.2 (latest: v2.6.0)
      Adding cfg_aliases v0.1.1 (latest: v0.2.1)
      Adding glow v0.13.1 (latest: v0.14.0)
      Adding gpu-allocator v0.26.0 (latest: v0.27.0)
      Adding jni-sys v0.3.0 (latest: v0.4.0)
      Adding malloc_buf v0.0.6 (latest: v1.0.0)
      Adding ndk-sys v0.5.0+25.2.9519653 (latest: v0.6.0+11769913)
      Adding rustc-hash v1.1.0 (latest: v2.0.0)
      Adding syn v1.0.109 (latest: v2.0.72)
      Adding windows v0.52.0 (latest: v0.58.0)
      Adding windows-core v0.52.0 (latest: v0.58.0)

```

cargo run -p xxx について

```
user: root …/Owner/work/private/test/cargo-workspace-test on  main [!] via 🦀 v1.80.0
❯ cargo run -p learning-wgpu/
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
error: package ID specification `learning-wgpu/` looks like a file path, maybe try file:///mnt/c/Users/Owner/work/private/test/cargo-workspace-test/learning-wgpu

user: root …/Owner/work/private/test/cargo-workspace-test on  main [!] via 🦀 v1.80.0
❯ cargo run -p learning-wgpu
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.03s
     Running `target/debug/learning-wgpu`
Hello, world!
```

#　作ったwasmを開く
test.htmlを用意してwasm-packでビルドしたwasmを開く。
```
cargo-workspace-test on  main [?] via 🦀 v1.80.1 on ☁️   
❯ python -m http.server
Serving HTTP on :: port 8000 (http://[::]:8000/) ...
::1 - - [10/Aug/2024 09:51:29] "GET / HTTP/1.1" 200 -
::1 - - [10/Aug/2024 09:51:29] code 404, message File not found
::1 - - [10/Aug/2024 09:51:29] "GET /favicon.ico HTTP/1.1" 404 -
::1 - - [10/Aug/2024 09:51:36] "GET /learning-wgpu/ HTTP/1.1" 200 -
::1 - - [10/Aug/2024 09:51:37] "GET /learning-wgpu/test.html HTTP/1.1" 200 -
::1 - - [10/Aug/2024 09:51:37] "GET /learning-wgpu/pkg/learning_wgpu.js HTTP/1.1" 200 -
::1 - - [10/Aug/2024 09:51:37] "GET /learning-wgpu/pkg/learning_wgpu_bg.wasm HTTP/1.1" 200 -
::1 - - [10/Aug/2024 09:51:37] "GET /learning-wgpu/pkg/learning_wgpu.js HTTP/1.1" 200 -

```
こうなった
![alt text](image.png)
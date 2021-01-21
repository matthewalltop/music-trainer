# Music Trainer

Music trainer is a desktop application built with Rust & GTK. It is intended as a tool for musicians to practice ear training and transcribing music. 

**This application is an early work in progress.**

I'll update progress in this ReadMe and eventually just rewrite it as an actual ReadMe once the app is in a more usable state. For now, here's the pain in the ass process to get GTK to work on Windows with Rust

## Get GTK working on Windows 10

***NOTE**, you also need MSYS64*

### Windows Steps
1. Install C++ build tools from https://visualstudio.microsoft.com/visual-cpp-build-tools/
2. Clone vcpkg into C:\ and bootstrap using instructions: https://github.com/microsoft/vcpkg
3. Add C:\vcpkg to %PATH%
4. Run vcpkg install gtk:x64-windows
5. Add C:\vcpkg\installed\x64-windows\bin to %PATH%
6. Set the GTK_LIB_DIR environment variable to C:\vcpkg\installed\x64-windows\lib.
7. From Powershell, run $ENV:PKG_CONFIG_ALLOW_CROSS=1

### Rust Steps
6. Copy all dependencies and features from the example here into Cargo.toml for project: https://github.com/gtk-rs/examples/blob/master/Cargo.toml
7. Run `rustup target add x86_64-pc-windows-gnu`
8. Run `cargo run --features gtk_3_22 --target=x86_64-pc-windows-gnu`



Alternatively it is much simpler to just copy `c:\msys64\mingw64\x86_64-w64-mingw32\lib\crt2.0` to `c:\Users%USER%.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-gnu\lib\`

crt2.o and dllcrt2.o have to be copied over as well.


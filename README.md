# Oxide

Oxide is a programmatic 3D engine written in Rust, with a C ABI and C# and Python bindings. The current graphics backend requires **64-bit Windows and Direct3D 12**.

## Prerequisites

- Rust stable, using the `x86_64-pc-windows-msvc` toolchain.
- Visual Studio 2022 Build Tools (or Visual Studio) with **Desktop development with C++** and a recent Windows SDK.
- DirectX Shader Compiler (`dxc.exe`), provided by recent Windows SDKs or the [official DXC releases](https://github.com/microsoft/DirectXShaderCompiler/releases). For a standalone download, keep `dxc.exe`, `dxcompiler.dll`, and `dxil.dll` together in its `bin/x64` directory.
- For C#: the .NET 10 SDK.
- For Python: 64-bit Python 3.11 or newer.

## Build and run

Run these commands in PowerShell from the repository root:

```powershell
.\scripts\build-shaders.ps1
cargo build --locked --lib --examples
cargo run --locked --example materials
```

The shader script searches `PATH`, then installed Windows SDKs. You can select a compiler explicitly:

```powershell
.\scripts\build-shaders.ps1 -DxcPath 'C:\Tools\dxc\bin\x64\dxc.exe'
```

If Windows blocks local PowerShell scripts, invoke it with a process-only policy override:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-shaders.ps1
```

The script compiles `vs_main` and `ps_main` in `triangle.hlsl`, `background.hlsl`,
`tonemap.hlsl`, `shadow.hlsl`, `camera_surface.hlsl`, and `sprite.hlsl` using Shader
Model 6.0. It writes `<name>_vs.dxil` and `<name>_ps.dxil` into `shaders/compiled`.
These generated files are ignored by Git and embedded into the native library at
compile time. **Re-run the shader script and rebuild Rust after changing HLSL.**
A failed shader compilation stops the script.

Rust examples: `hello_window`, `cube`, `model`, `attachment`, `split_screen`, `first_person`, `materials`, `shadows`, `audio`, `monitors`, and `menus`. Run from the repository root so relative asset paths resolve correctly.

### C#

Build the native library first, then run an example. The example project copies `target/debug/oxide.dll` into its output directory.

```powershell
cargo build --locked --lib
dotnet run --project examples/csharp/Materials/Materials.csproj
```

`HelloWindow`, `FirstPerson`, `Shadows`, `Audio`, `Monitors`, and `Menus` are also available under `examples/csharp`.

### Python

Install the bindings once into your active Python environment after building the native library:

```powershell
cargo build --locked --lib
python -m pip install --editable .\bindings\python
python .\examples\python\materials.py
```

The development bindings look for `target/debug/oxide.dll`, then `target/release/oxide.dll`. To select another build, set `OXIDE_LIBRARY` to the absolute DLL path. The Python package installation does not compile or bundle the native engine.

The editable install persists: run the example directly on later sessions.
Python source changes are picked up automatically; Rust changes require a native
library rebuild.

### C

From an **x64 Native Tools Command Prompt for VS 2022**, at the repository root, after building the native library:

```bat
cl /nologo /std:c11 /W4 /Fe:target\debug\hello_window_c.exe /Fo:target\debug\hello_window_c.obj examples\c\hello_window.c /link /LIBPATH:target\debug oxide.dll.lib
target\debug\hello_window_c.exe
```

The executable is placed beside `oxide.dll` so Windows can load it.

## API documentation

Public Rust APIs use Rustdoc, C# APIs use XML documentation, Python APIs expose
docstrings through `help()`, and `include/oxide.h` documents the C ABI with
Doxygen comments. See [documentation and style conventions](docs/contributing.md)
for the documentation-build and formatting commands.

## Automated checks

All test code and runners live under [`tests/`](tests/README.md). After compiling the shaders:

```powershell
python tests/check_test_layout.py
cargo check --locked --all-targets
cargo test --locked --lib --tests
cargo build --locked --lib --examples
dotnet build bindings/csharp/Oxide.NET/Oxide.NET.csproj
dotnet run --project tests/csharp/Oxide.Tests.csproj
Get-ChildItem examples/csharp -Recurse -Filter *.csproj | ForEach-Object {
    dotnet build $_.FullName
    if ($LASTEXITCODE -ne 0) { throw "C# build failed: $($_.FullName)" }
}
python -m pip install --editable .\bindings\python
python -m compileall -q bindings/python/oxide examples/python tests
python -c "import oxide; import oxide.app; import oxide.collision; import oxide.content; import oxide.graphics; import oxide.input; import oxide.math"
python tests/python_math.py
python tests/python_shadows.py
python tests/python_audio.py
python tests/python_monitors.py
python tests/python_sprites.py
```

[Windows CI](https://github.com/HudsonBakke/Oxide/actions/workflows/windows.yml) runs on pushes, pull requests, and manual dispatch. It compiles all ten shader stages, checks every Rust target, runs Rust tests (including D3D12 shadow rendering and HDR readback on WARP), builds the native library and all Rust/C# examples, compiles and links the C example, and checks Python syntax, imports, math checks, and the shadow ABI and example controls against the freshly built DLL. Python is checked at the package's minimum supported version, 3.11.

CI checks rendered HDR output through headless WARP readbacks; it does not launch interactive windows. Run the materials, first-person, and shadows scenes locally to check rendering and input on a D3D12-capable GPU.

## Linear HDR rendering

Lighting and backgrounds render into an RGBA16F linear scene target. One final
Reinhard tone-mapping pass applies exposure and encodes sRGB for the SDR window.
See [rendering and color conventions](docs/rendering.md) for texture semantics
and the exposure API. In each materials example, **Q** halves exposure, **E**
doubles it, and **R** resets it to one. The title shows the current multiplier.

See [light-owned shadows and transparency](docs/shadows.md) for automatic shadows through DrawMesh, scalar transparency, coverage and bias settings, and matching Rust, C#, and Python demos.

See [GPU resource lifetime and capacity](docs/resources.md) for cache reclamation,
reusable descriptors, upload pages, and renderer limits.

## Audio

Load WAV/MP3 as `Audio`, play through a positional `AudioSource`, select an
independent `AudioListener`, and modify shared `AudioSettings` during playback.
Both omnidirectional and cone emission use distance attenuation. Volume, pitch,
looping, low-pass filtering, and amplitude modulation are supported. Occlusion
is application-controlled. See [the audio guide](docs/audio.md) for API examples,
ownership, limits, and matching Rust/C#/Python listening demos.

## Camera surfaces

Bind a camera to a validated world-space `Rect3D` to create a live monitor image
that appears automatically in screen-bound cameras. See [camera surfaces](docs/camera-surfaces.md)
for matching Rust/C#/Python APIs, image sizing, front-face behavior, and monitor demos.

## Screen-space graphics

Draw textures and Unicode labels over a scene with `Draw2D` and `DrawText`, or use them alone for a 2D menu. See [2D drawing](docs/drawing-2d.md) and the matching `menus` examples in Rust, C#, and Python.

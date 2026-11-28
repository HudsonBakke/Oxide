# Tests

Every test, fixture, helper, and test runner lives in this directory. Production
`src/` contains no test modules, test attributes, or test-only implementations.
Run commands from the repository root on 64-bit Windows.

```powershell
python tests/check_test_layout.py
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-shaders.ps1
cargo test --locked --lib --tests -- --test-threads=1
cargo build --locked --lib
# An existing editable install needs no reinstall.
python tests/python_math.py
python tests/python_shadows.py
python tests/python_audio.py
python tests/python_monitors.py
dotnet run --project tests/csharp/Oxide.Tests.csproj
```

Install the Python bindings once with `python -m pip install --editable
./bindings/python`. `OXIDE_LIBRARY` can select a specific native DLL; otherwise
the package discovers the development DLL as described in the root README.

Rust public-API checks are the top-level `.rs` integration targets. Private
implementation checks use `internal.rs` and the small module wrappers under
`internal/`. These wrappers compile the actual production files using `#[path]`
and `include!`; they do not duplicate rendering algorithms or introduce a public
inspection API. Keep their module declarations aligned when adding or moving
production modules. All assertions and the WARP-device helper are in `tests/`.

Headless D3D12 tests create real pipelines, render on WARP, and read HDR pixels
back to verify visible transparency and point/spot/directional shadow
transmission. Lifetime tests drop CPU assets before execution, repeatedly cycle
more than a heap's worth of descriptors, check clone/environment ownership,
and verify that capacity errors preserve earlier draws. Upload checks exceed
4 MiB, preserve earlier data, and exercise page reuse/retirement. Allocator
checks cover fragmentation, coalescing, exhaustion, and invalid releases.

`python_shadows.py` checks the native ABI, shared light interface, and example
controls. `python_math.py` is a native-binding math smoke check.
`csharp/Oxide.Tests.csproj` checks shared light properties and native struct sizes.

The interactive input/parity checks were moved from the Python examples:

```powershell
python tests/python_input_manual.py
python tests/python_parity_manual.py
```

These open windows and require manual observation. CI runs the automated checks,
builds every language example, and leaves interactive window/input checks to the
developer. `run_rust_tests.py` captures the Rust runner's output for CI failure
summaries; `--check` captures `cargo check --all-targets` instead.

Audio coverage includes WAV/MP3 decoding and cache reuse/invalidation, corrupt
cache recovery, settings validation, listener orientation, distance/cone gain,
panning, sample-rate conversion, pitch, looping/completion, clip ownership,
low-pass filtering, modulation, and native C#/Python layouts/handles. These
checks need no playback endpoint. All listening checks are the audio examples;
run them on a machine with an active Windows output device. The hex MP3 fixture
is a generated sine wave, not third-party music. The Rust runner's --doc option
captures documentation failures as well as --check compile failures.

Rect3D math/ABI tests validate rectangle geometry and normal selection. Camera
surface WARP tests validate current-frame scheduling, independent feeds and screen
viewports, UV orientation, depth/alpha behavior, HDR preservation, resource reuse,
resizing, capacity errors, and retirement. Python tests exercise the native value
layout and actual example controls; the C# test program checks native construction
and all camera overloads. Interactive demos live under examples, with all test
assertions and helpers kept here.

The sprite checks read back the actual sRGB display and overlay passes under WARP. They cover linear alpha blending, scene exposure isolation, ordering, crops, flips, pivot rotation, clipping, text coverage, and dropped CPU assets. Font cache and binding checks cover Unicode, measurement, validation, ABI layout and the menu example controls. Run `python tests/python_sprites.py` alongside the other Python checks.

# Positional audio

Audio is independent of graphics. `Audio` is an immutable decoded asset;
`AudioSource` is an emitter with one playback; `AudioSettings` is a shared mutable
set of controls; `AudioListener` is the viewpoint from which sources are heard.

The Windows backend uses XAudio2 and Media Foundation. No additional native SDK,
Python package, or NuGet dependency is needed beyond the existing Windows build
requirements. WAV and MP3 are supported; Windows N editions require their Media
Feature Pack to supply the Media Foundation runtime used by the native library. Creating assets, settings, and listeners does not
open an output endpoint. Constructing the first source opens the default device;
a missing endpoint returns an audio error. Restore the device and recreate all
sources after an output failure. Automatic device switching is not implemented.

## C#

```csharp
using Oxide.Audio;
using Oxide.Content;
using Oxide.Math;
using Clip = Oxide.Audio.Audio;

ContentManager content = new();
using Clip audio = content.Load<Clip>("assets/myaudio.mp3");
using AudioSettings settings = new() { Volume = 0.8f, Looping = true };
using AudioListener listener = new();
listener.Activate();
using AudioSource source = new(new Vec3(0, 0, -3), settings, new Vec3(0, 0, 1));
source.Play(audio);

// Changes apply to playback without Play being called again.
source.Settings.Volume = 0.5f;
source.Settings.LowPassCutoff = 800; // Application-controlled muffling
source.Position = new Vec3(2, 0, -3);
source.PlayDirectional(audio, Angle.Degrees(15), Angle.Degrees(40));
source.Pause();
source.Resume();
source.Stop();
```

Keep owners alive for the intended playback duration. Disposing a source stops
it; disposing the original clip/settings wrappers does not invalidate an active
source's native ownership. The source's Settings property can obtain another
handle if its original settings wrapper was disposed.

## Python

```python
from oxide import Audio, AudioSettings, AudioSource, AudioListener, ContentManager, Vec3, Angle

with ContentManager().load(Audio, "assets/myaudio.mp3") as audio:
    with AudioSettings(volume=0.8, looping=True) as settings, AudioListener() as listener:
        listener.activate()
        with AudioSource(Vec3(0, 0, -3), settings, Vec3(0, 0, 1)) as source:
            source.play(audio)
            # Keep the source alive in your game object; perform these edits in Update.
            source.settings.volume = 0.5
            source.settings.low_pass_cutoff = 800
            source.play_directional(audio, Angle.degrees(15), Angle.degrees(40))
```

Leaving the source context stops playback. Playback runs on a Rust worker,
independently of Python callbacks and rendering. `close()` releases wrappers
explicitly; otherwise collection releases them.

## Rust

```rust,ignore
use oxide::{audio::{Audio, AudioSettings, AudioSource, AudioListener}, content::ContentManager, math::{Vec3, Angle}};

let audio = ContentManager::new().load::<Audio>("assets/myaudio.mp3")?;
let listener = AudioListener::new();
listener.activate();
let settings = AudioSettings::new();
settings.set_looping(true);
let source = AudioSource::new(Vec3::new(0.0, 0.0, -3.0), settings.clone(), Vec3::new(0.0, 0.0, 1.0))?;
source.play(&audio)?;
settings.set_volume(0.5)?;
settings.set_low_pass_cutoff(800.0)?;
source.play_directional(&audio, Angle::degrees(15.0), Angle::degrees(40.0), 0.0)?;
// Store source in your game state. Dropping it stops playback.
```

Cloning Audio shares decoded samples. Cloning AudioSettings shares live controls;
create a new settings object for independent controls. Playing retains a clip
clone, so dropping the original asset is safe. Play/PlayDirectional replace the
source's existing sound; use multiple sources to overlap sounds. Pause retains
the clip/cursor, Resume continues it, and Stop resets the cursor and releases the
clip. Nonlooping completion releases the playback's clip automatically. Reported
state follows the mixer; a small amount of already queued output may remain.

## Listener and spatial behavior

The initial active listener is at the origin, facing -Z with +Y up. Constructing
a listener does not activate it. Activate selects it once; position/orientation
edits then apply live. Activation retains native listener state until another
listener is selected. Position units are the same as your world's units. Forward
and up must be finite, nonzero, and nonparallel; orientation is orthonormalized.

To follow a camera, update listener position from camera position and rotate -Z
and +Y by the camera rotation before calling SetOrientation/set_orientation.
There is one active listener; the application chooses the viewpoint in
split-screen scenes. No camera/scene/Model abstraction is required.

Both modes have distance attenuation and equal-power stereo panning relative to
the listener. Below reference distance, distance gain is one. At or beyond maximum
distance it is zero. Between the bounds, gain is
`(reference / distance)^rolloff * (maximum - distance) / (maximum - reference)`.
Inaudible sources continue advancing their cursors.

Directional angles are **half-angles**, with
`0 <= inner <= outer <= 180 degrees`. Inside the inner cone, cone gain is one;
between the cones it fades linearly to outerVolume; outside the outer cone it is
outerVolume. Equal inner/outer angles produce a hard edge. A listener at the
source position has full cone gain. SetCone changes emission without restarting.
Play resets emission to omnidirectional.

## Live settings

| Setting (C# / Python) | Default | Meaning |
| --- | --- | --- |
| Volume / volume | 1 | Linear amplitude, 0..16; zero mutes |
| Pitch / pitch | 1 | Playback-rate ratio, 1/16..16; changes pitch and duration together |
| Looping / looping | false | Wrap to beginning; turning it off lets the current traversal finish |
| ReferenceDistance / reference_distance | 1 | 0.0001..1e9 world units, where attenuation starts |
| MaxDistance / max_distance | 50 | 0.0002..1e12 world units, strictly above reference distance |
| Rolloff / rolloff | 1 | Inverse-distance exponent, 0..16, with the range fade still applied |
| LowPassCutoff / low_pass_cutoff | 24000 | First-order low-pass Hz, 20..24000; 24000 bypasses it |
| ModulationFrequency / modulation_frequency | 0 | Tremolo frequency in Hz, 0..100; zero disables it |
| ModulationDepth / modulation_depth | 0 | Tremolo depth, 0..1; zero disables it |

SetDistanceRange/set_distance_range updates both bounds atomically. All numeric
values must be finite. Invalid changes leave settings unchanged. Volume, pitch,
filter coefficient, and modulation controls interpolate over a mixer block to
reduce discontinuities. Moving sources and listener orientation use the same gain
smoothing. Pause/Stop/replacement are discrete playback operations, rather than
sample-accurate crossfades.

There is **no automatic occlusion and no occlusion/dampening flag**. Applications
can raycast using their own geometry and change Volume/LowPassCutoff as desired.
The audio subsystem does not depend on collision or draw submissions.

## Initial scope and output

Clips are fully decoded and cached as mono 32-bit float samples, limited to
67,108,864 samples (256 MiB decoded). Stereo/multichannel file channels are averaged
to mono for positional playback. The mixer outputs stereo at 48 kHz and uses
linear interpolation for rate conversion/pitch, a first-order low-pass, and
amplitude modulation. It is an initial positional effects system: no music
streaming, HRTF, Doppler, reverb, time-stretching, or multichannel spatial output.
Summed output is clamped to [-1,1]; reduce source volumes to avoid clipping.

Three 10 ms buffers are queued to XAudio2, with additional latency determined by
the device/OS. Device operations are confined to the worker thread; output
buffers are freed only after XAudio2 returns ownership or DestroyVoice completes.
The worker and device are released after the final source is dropped.

See the native [XAudio2 streaming documentation](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-streaming-audio-data)
and [Media Foundation decoding tutorial](https://learn.microsoft.com/en-us/windows/win32/medfound/tutorial--decoding-audio)
for backend API details.

## Run the matching demos

Rebuild the native library, then run one demo at a time:

```powershell
cargo build --locked --lib --examples
cargo run --locked --example audio
dotnet run --project examples/csharp/Audio/Audio.csproj
python .\examples\python\audio.py
```

Each demo generates a quiet tone and moves it left/right, then switches to a
rotating cone while modifying pitch, filtering, and tremolo. Headphones make
panning easier to hear. An optional argument loads WAV/MP3 instead:

```powershell
cargo run --locked --example audio -- "assets/myaudio.mp3"
dotnet run --project examples/csharp/Audio/Audio.csproj -- "assets/myaudio.mp3"
python .\examples\python\audio.py "assets/myaudio.mp3"
```

An existing editable Python install needs no reinstall after rebuilding oxide.dll.
All automated audio tests and fixtures are under tests/. Decoder, mixer, settings,
and ABI tests run without an audio endpoint; listening/device verification uses
the demos locally. The MP3 fixture is an FFmpeg-generated 440 Hz sine wave,
stored as hex text so no binary test data needs special handling.

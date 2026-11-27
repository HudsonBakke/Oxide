"""Native audio asset/settings/listener checks; requires no playback endpoint."""
import ctypes
import math
from pathlib import Path
import struct
import tempfile
import unittest
import wave
from oxide import Audio, AudioSettings, AudioListener, AudioSource, AudioError, ContentManager, ContentError, InvalidArgumentError, Vec3
from oxide.audio import _SettingsValues
from oxide import _native

class AudioBindingsTests(unittest.TestCase):
    def test_samples_metadata_and_closed_handles(self):
        with Audio.from_samples([0.0,0.5,-1.0],24000) as clip:
            self.assertEqual(clip.sample_count,3)
            self.assertEqual(clip.sample_rate,24000)
            self.assertAlmostEqual(clip.duration,3/24000)
        with self.assertRaises(RuntimeError):
            _=clip.duration
        for samples in [[],[float("nan")],[1.1]]:
            with self.assertRaises(InvalidArgumentError):
                Audio.from_samples(samples)

    def test_live_settings_layout_validation_and_atomic_constructor(self):
        self.assertEqual(ctypes.sizeof(_SettingsValues),36)
        with AudioSettings(volume=0.5,looping=True,reference_distance=10,max_distance=20) as settings:
            self.assertEqual(settings.volume,0.5)
            self.assertTrue(settings.looping)
            self.assertEqual(settings.reference_distance,10)
            settings.low_pass_cutoff=400
            settings.modulation_frequency=6
            settings.modulation_depth=0.8
            for value in [-1,float("nan"),float("inf"),17]:
                with self.assertRaises(InvalidArgumentError):
                    settings.volume=value
            self.assertEqual(settings.volume,0.5)
            with self.assertRaises(InvalidArgumentError):
                settings.set_distance_range(20,10)
            self.assertEqual(settings.reference_distance,10)
            with self.assertRaises(InvalidArgumentError):
                AudioSource(Vec3(float("nan"),0,0),settings)
        with self.assertRaises(TypeError):
            AudioSettings(unknown=1)

    def test_listener_edits_and_invalid_orientation(self):
        with AudioListener() as listener:
            listener.activate()
            listener.position=Vec3(1,2,3)
            self.assertEqual(listener.position,Vec3(1,2,3))
            listener.set_orientation(Vec3(1,0,0),Vec3(0,2,0))
            self.assertEqual(listener.direction,Vec3(1,0,0))
            self.assertAlmostEqual(listener.up.length,1)
            with self.assertRaises(InvalidArgumentError):
                listener.set_orientation(Vec3(),Vec3(0,1,0))
            self.assertEqual(listener.direction,Vec3(1,0,0))

    def test_wav_mp3_content_and_cache(self):
        with tempfile.TemporaryDirectory(prefix="oxide-audio-") as directory:
            root=Path(directory)
            wav_path=root/"tone-音声.wav"
            with wave.open(str(wav_path),"wb") as output:
                output.setparams((2,2,44100,0,"NONE","not compressed"))
                output.writeframes(struct.pack("<hh",16384,0)*100)
            content=ContentManager(str(root/"cache"))
            with content.load(Audio,wav_path) as clip:
                self.assertEqual(clip.sample_count,100)
            cache=next((root/"cache").iterdir())
            modified=cache.stat().st_mtime_ns
            with content.load(Audio,wav_path) as clip:
                self.assertEqual(clip.sample_rate,44100)
            self.assertEqual(cache.stat().st_mtime_ns,modified)
            fixture=Path(__file__).parent/"fixtures"/"tone_mp3.hex"
            mp3_path=root/"tone.mp3"
            mp3_path.write_bytes(bytes.fromhex(fixture.read_text()))
            with content.load(Audio,mp3_path) as clip:
                self.assertGreater(clip.duration,0.15)
            with self.assertRaises(ContentError):
                content.load(Audio,wav_path,"track")
            bad=root/"bad.mp3"
            bad.write_bytes(b"not audio")
            with self.assertRaises(ContentError):
                content.load(Audio,bad)

if __name__=="__main__":
    unittest.main()

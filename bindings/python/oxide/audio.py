"""Positional audio, live settings, and an independent listener. No automatic occlusion."""
from __future__ import annotations
import ctypes
import weakref
from enum import IntEnum
from . import _native
from .math import Angle, Vec3

class _SettingsValues(ctypes.Structure):
    _fields_ = [
        ("volume", ctypes.c_float),
        ("pitch", ctypes.c_float),
        ("reference_distance", ctypes.c_float),
        ("max_distance", ctypes.c_float),
        ("rolloff", ctypes.c_float),
        ("low_pass_cutoff", ctypes.c_float),
        ("modulation_frequency", ctypes.c_float),
        ("modulation_depth", ctypes.c_float),
        ("looping", ctypes.c_uint32),
    ]

# Bind the C ABI here to keep _native independent of public audio classes.
_native.lib.oxide_audio_destroy.argtypes = [ctypes.c_uint64]
_native.lib.oxide_audio_destroy.restype = ctypes.c_int
_native.lib.oxide_audio_settings_destroy.argtypes = [ctypes.c_uint64]
_native.lib.oxide_audio_settings_destroy.restype = ctypes.c_int
_native.lib.oxide_audio_source_destroy.argtypes = [ctypes.c_uint64]
_native.lib.oxide_audio_source_destroy.restype = ctypes.c_int
_native.lib.oxide_audio_listener_destroy.argtypes = [ctypes.c_uint64]
_native.lib.oxide_audio_listener_destroy.restype = ctypes.c_int
_native.lib.oxide_audio_create_samples.argtypes = [ctypes.POINTER(ctypes.c_float), ctypes.c_size_t, ctypes.c_uint32, ctypes.POINTER(ctypes.c_uint64)]
_native.lib.oxide_audio_create_samples.restype = ctypes.c_int
_native.lib.oxide_audio_metadata.argtypes = [ctypes.c_uint64, ctypes.POINTER(ctypes.c_uint32), ctypes.POINTER(ctypes.c_uint64), ctypes.POINTER(ctypes.c_double)]
_native.lib.oxide_audio_metadata.restype = ctypes.c_int
_native.lib.oxide_audio_settings_create.argtypes = [ctypes.POINTER(ctypes.c_uint64)]
_native.lib.oxide_audio_settings_create.restype = ctypes.c_int
_native.lib.oxide_audio_settings_get.argtypes = [ctypes.c_uint64, ctypes.POINTER(_SettingsValues)]
_native.lib.oxide_audio_settings_get.restype = ctypes.c_int
_native.lib.oxide_audio_settings_set.argtypes = [ctypes.c_uint64, _SettingsValues]
_native.lib.oxide_audio_settings_set.restype = ctypes.c_int
_native.lib.oxide_audio_settings_set_distance_range.argtypes = [ctypes.c_uint64, ctypes.c_float, ctypes.c_float]
_native.lib.oxide_audio_settings_set_distance_range.restype = ctypes.c_int
_native.lib.oxide_audio_settings_set_looping.argtypes = [ctypes.c_uint64, ctypes.c_uint32]
_native.lib.oxide_audio_settings_set_looping.restype = ctypes.c_int
_native.lib.oxide_audio_settings_set_volume.argtypes = [ctypes.c_uint64, ctypes.c_float]
_native.lib.oxide_audio_settings_set_volume.restype = ctypes.c_int
_native.lib.oxide_audio_settings_set_pitch.argtypes = [ctypes.c_uint64, ctypes.c_float]
_native.lib.oxide_audio_settings_set_pitch.restype = ctypes.c_int
_native.lib.oxide_audio_settings_set_reference_distance.argtypes = [ctypes.c_uint64, ctypes.c_float]
_native.lib.oxide_audio_settings_set_reference_distance.restype = ctypes.c_int
_native.lib.oxide_audio_settings_set_max_distance.argtypes = [ctypes.c_uint64, ctypes.c_float]
_native.lib.oxide_audio_settings_set_max_distance.restype = ctypes.c_int
_native.lib.oxide_audio_settings_set_rolloff.argtypes = [ctypes.c_uint64, ctypes.c_float]
_native.lib.oxide_audio_settings_set_rolloff.restype = ctypes.c_int
_native.lib.oxide_audio_settings_set_low_pass_cutoff.argtypes = [ctypes.c_uint64, ctypes.c_float]
_native.lib.oxide_audio_settings_set_low_pass_cutoff.restype = ctypes.c_int
_native.lib.oxide_audio_settings_set_modulation_frequency.argtypes = [ctypes.c_uint64, ctypes.c_float]
_native.lib.oxide_audio_settings_set_modulation_frequency.restype = ctypes.c_int
_native.lib.oxide_audio_settings_set_modulation_depth.argtypes = [ctypes.c_uint64, ctypes.c_float]
_native.lib.oxide_audio_settings_set_modulation_depth.restype = ctypes.c_int
_native.lib.oxide_audio_source_create.argtypes = [Vec3, ctypes.c_uint64, Vec3, ctypes.POINTER(ctypes.c_uint64)]
_native.lib.oxide_audio_source_create.restype = ctypes.c_int
_native.lib.oxide_audio_source_play.argtypes = [ctypes.c_uint64, ctypes.c_uint64]
_native.lib.oxide_audio_source_play.restype = ctypes.c_int
_native.lib.oxide_audio_source_play_directional.argtypes = [ctypes.c_uint64, ctypes.c_uint64, Angle, Angle, ctypes.c_float]
_native.lib.oxide_audio_source_play_directional.restype = ctypes.c_int
_native.lib.oxide_audio_source_set_cone.argtypes = [ctypes.c_uint64, Angle, Angle, ctypes.c_float]
_native.lib.oxide_audio_source_set_cone.restype = ctypes.c_int
_native.lib.oxide_audio_source_set_settings.argtypes = [ctypes.c_uint64, ctypes.c_uint64]
_native.lib.oxide_audio_source_set_settings.restype = ctypes.c_int
_native.lib.oxide_audio_source_get_settings.argtypes = [ctypes.c_uint64, ctypes.POINTER(ctypes.c_uint64)]
_native.lib.oxide_audio_source_get_settings.restype = ctypes.c_int
_native.lib.oxide_audio_source_get_state.argtypes = [ctypes.c_uint64, ctypes.POINTER(ctypes.c_uint32)]
_native.lib.oxide_audio_source_get_state.restype = ctypes.c_int
_native.lib.oxide_audio_source_pause.argtypes = [ctypes.c_uint64]
_native.lib.oxide_audio_source_pause.restype = ctypes.c_int
_native.lib.oxide_audio_source_resume.argtypes = [ctypes.c_uint64]
_native.lib.oxide_audio_source_resume.restype = ctypes.c_int
_native.lib.oxide_audio_source_stop.argtypes = [ctypes.c_uint64]
_native.lib.oxide_audio_source_stop.restype = ctypes.c_int
_native.lib.oxide_audio_listener_create.argtypes = [ctypes.POINTER(ctypes.c_uint64)]
_native.lib.oxide_audio_listener_create.restype = ctypes.c_int
_native.lib.oxide_audio_listener_activate.argtypes = [ctypes.c_uint64]
_native.lib.oxide_audio_listener_activate.restype = ctypes.c_int
_native.lib.oxide_audio_listener_set_orientation.argtypes = [ctypes.c_uint64, Vec3, Vec3]
_native.lib.oxide_audio_listener_set_orientation.restype = ctypes.c_int
_native.lib.oxide_audio_source_get_position.argtypes = [ctypes.c_uint64, ctypes.POINTER(Vec3)]
_native.lib.oxide_audio_source_get_position.restype = ctypes.c_int
_native.lib.oxide_audio_source_set_position.argtypes = [ctypes.c_uint64, Vec3]
_native.lib.oxide_audio_source_set_position.restype = ctypes.c_int
_native.lib.oxide_audio_source_get_direction.argtypes = [ctypes.c_uint64, ctypes.POINTER(Vec3)]
_native.lib.oxide_audio_source_get_direction.restype = ctypes.c_int
_native.lib.oxide_audio_source_set_direction.argtypes = [ctypes.c_uint64, Vec3]
_native.lib.oxide_audio_source_set_direction.restype = ctypes.c_int
_native.lib.oxide_audio_listener_get_position.argtypes = [ctypes.c_uint64, ctypes.POINTER(Vec3)]
_native.lib.oxide_audio_listener_get_position.restype = ctypes.c_int
_native.lib.oxide_audio_listener_set_position.argtypes = [ctypes.c_uint64, Vec3]
_native.lib.oxide_audio_listener_set_position.restype = ctypes.c_int
_native.lib.oxide_audio_listener_get_direction.argtypes = [ctypes.c_uint64, ctypes.POINTER(Vec3)]
_native.lib.oxide_audio_listener_get_direction.restype = ctypes.c_int
_native.lib.oxide_audio_listener_get_up.argtypes = [ctypes.c_uint64, ctypes.POINTER(Vec3)]
_native.lib.oxide_audio_listener_get_up.restype = ctypes.c_int
_native.lib.oxide_content_load_audio.argtypes = [_native.NativeContentManagerHandle, ctypes.c_char_p, ctypes.POINTER(ctypes.c_uint64)]
_native.lib.oxide_content_load_audio.restype = ctypes.c_int
_native.lib.oxide_content_load_audio_sub.argtypes = [_native.NativeContentManagerHandle, ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_uint64)]
_native.lib.oxide_content_load_audio_sub.restype = ctypes.c_int

class _Resource:
    _destroy = None

    def _attach(self, handle):
        self._handle = handle
        self._finalizer = weakref.finalize(self, type(self)._destroy, handle)

    @classmethod
    def _from_handle(cls, handle):
        result = cls.__new__(cls)
        result._attach(handle)
        return result

    def _value(self):
        if not self._finalizer.alive:
            raise RuntimeError("audio resource is closed")
        return self._handle

    def close(self):
        """Releases this handle; safe to call repeatedly. Active sources retain their data."""
        self._finalizer()

    def __enter__(self):
        """Keeps the resource open for the context block."""
        self._value()
        return self

    def __exit__(self, *args):
        """Releases this handle on leaving the context block."""
        self.close()

class Audio(_Resource):
    """Immutable mono samples loaded as WAV/MP3 through ContentManager; playback retains data.

    Multichannel files are averaged to mono for positional emission. Assets are fully decoded.
    """
    _destroy = _native.lib.oxide_audio_destroy

    @classmethod
    def from_samples(cls, samples, sample_rate: int = 48000) -> Audio:
        """Copies nonempty finite mono samples in [-1,1], at 8000..192000 Hz; at most 64 million samples."""
        data = (ctypes.c_float * len(samples))(*samples)
        handle = ctypes.c_uint64()
        _native.check(_native.lib.oxide_audio_create_samples(data, len(samples), sample_rate, ctypes.byref(handle)))
        return cls._from_handle(handle)

    def _metadata(self):
        rate, count, duration = ctypes.c_uint32(), ctypes.c_uint64(), ctypes.c_double()
        _native.check(_native.lib.oxide_audio_metadata(self._value(), ctypes.byref(rate), ctypes.byref(count), ctypes.byref(duration)))
        return rate.value, count.value, duration.value

    @property
    def sample_rate(self) -> int:
        """Decoded mono samples per second."""
        return self._metadata()[0]

    @property
    def sample_count(self) -> int:
        """Number of decoded mono samples."""
        return self._metadata()[1]

    @property
    def duration(self) -> float:
        """Duration in seconds before pitch changes or looping."""
        return self._metadata()[2]

class AudioSettings(_Resource):
    """Shared live controls; changing this object updates every source using it."""
    _destroy = _native.lib.oxide_audio_settings_destroy

    def __init__(self, **values):
        """Creates default controls, optionally overriding named properties atomically.

        volume/pitch default to one, reference_distance=1, max_distance=50,
        rolloff=1, low_pass_cutoff=24000, modulation frequency/depth=0, looping=False.
        """
        handle = ctypes.c_uint64()
        _native.check(_native.lib.oxide_audio_settings_create(ctypes.byref(handle)))
        self._attach(handle)
        if values:
            data = self._snapshot()
            allowed = {name for name, _ in _SettingsValues._fields_}
            for name, value in values.items():
                if name not in allowed:
                    raise TypeError(f"unknown audio setting: {name}")
                setattr(data, name, int(bool(value)) if name == "looping" else float(value))
            _native.check(_native.lib.oxide_audio_settings_set(self._value(), data))

    def _snapshot(self):
        value = _SettingsValues()
        _native.check(_native.lib.oxide_audio_settings_get(self._value(), ctypes.byref(value)))
        return value

    def set_distance_range(self, reference: float, maximum: float) -> None:
        """Sets both bounds atomically; requires 0 < reference < maximum."""
        _native.check(_native.lib.oxide_audio_settings_set_distance_range(self._value(), reference, maximum))

    @property
    def volume(self) -> float:
        """Linear gain in [0,16], initially one. Edits affect active playback."""
        return self._snapshot().volume

    @volume.setter
    def volume(self, value: float) -> None:
        _native.check(_native.lib.oxide_audio_settings_set_volume(self._value(), float(value)))

    @property
    def pitch(self) -> float:
        """Playback-rate ratio in [1/16,16], initially one. Edits affect active playback."""
        return self._snapshot().pitch

    @pitch.setter
    def pitch(self, value: float) -> None:
        _native.check(_native.lib.oxide_audio_settings_set_pitch(self._value(), float(value)))

    @property
    def reference_distance(self) -> float:
        """World distance where attenuation starts, initially one. Edits affect active playback."""
        return self._snapshot().reference_distance

    @reference_distance.setter
    def reference_distance(self, value: float) -> None:
        _native.check(_native.lib.oxide_audio_settings_set_reference_distance(self._value(), float(value)))

    @property
    def max_distance(self) -> float:
        """World cutoff distance, greater than reference_distance; initially 50. Edits affect active playback."""
        return self._snapshot().max_distance

    @max_distance.setter
    def max_distance(self, value: float) -> None:
        _native.check(_native.lib.oxide_audio_settings_set_max_distance(self._value(), float(value)))

    @property
    def rolloff(self) -> float:
        """Inverse-distance exponent in [0,16], initially one. Edits affect active playback."""
        return self._snapshot().rolloff

    @rolloff.setter
    def rolloff(self, value: float) -> None:
        _native.check(_native.lib.oxide_audio_settings_set_rolloff(self._value(), float(value)))

    @property
    def low_pass_cutoff(self) -> float:
        """Low-pass cutoff in Hz, [20,24000]; 24000 bypasses filtering. Edits affect active playback."""
        return self._snapshot().low_pass_cutoff

    @low_pass_cutoff.setter
    def low_pass_cutoff(self, value: float) -> None:
        _native.check(_native.lib.oxide_audio_settings_set_low_pass_cutoff(self._value(), float(value)))

    @property
    def modulation_frequency(self) -> float:
        """Amplitude-modulation rate in Hz, [0,100]; zero disables it. Edits affect active playback."""
        return self._snapshot().modulation_frequency

    @modulation_frequency.setter
    def modulation_frequency(self, value: float) -> None:
        _native.check(_native.lib.oxide_audio_settings_set_modulation_frequency(self._value(), float(value)))

    @property
    def modulation_depth(self) -> float:
        """Amplitude-modulation depth in [0,1]; zero disables it. Edits affect active playback."""
        return self._snapshot().modulation_depth

    @modulation_depth.setter
    def modulation_depth(self, value: float) -> None:
        _native.check(_native.lib.oxide_audio_settings_set_modulation_depth(self._value(), float(value)))

    @property
    def looping(self) -> bool:
        """Whether playback wraps to the beginning, initially False."""
        return bool(self._snapshot().looping)

    @looping.setter
    def looping(self, value: bool) -> None:
        _native.check(_native.lib.oxide_audio_settings_set_looping(self._value(), int(bool(value))))

class PlaybackState(IntEnum):
    """State of the source's single playback."""
    #: No active clip, or a nonlooping clip has finished.
    STOPPED = 0
    #: Samples are advancing, including outside audible range.
    PLAYING = 1
    #: The clip and cursor are retained without advancing.
    PAUSED = 2

class AudioSource(_Resource):
    """Positional emitter with one playback; multiple sources can share a clip/settings.

    Output opens lazily at construction. Closing/collecting the source stops it.
    No geometry or occlusion is processed automatically.
    """
    _destroy = _native.lib.oxide_audio_source_destroy

    def __init__(self, position: Vec3, settings: AudioSettings | None = None, direction: Vec3 | None = None):
        """Creates a source, with default settings and -Z direction when omitted.

        Raises AudioError if an output device cannot be opened.
        """
        if settings is None:
            settings = AudioSettings()
        if not isinstance(settings, AudioSettings):
            raise TypeError("settings must be AudioSettings")
        handle = ctypes.c_uint64()
        _native.check(_native.lib.oxide_audio_source_create(position, settings._value(), direction if direction is not None else Vec3(0, 0, -1), ctypes.byref(handle)))
        self._attach(handle)
        self._settings = settings

    @property
    def settings(self) -> AudioSettings:
        """Shared controls; edits affect active and future playback."""
        self._value()
        if not self._settings._finalizer.alive:
            handle = ctypes.c_uint64()
            _native.check(_native.lib.oxide_audio_source_get_settings(self._value(), ctypes.byref(handle)))
            self._settings = AudioSettings._from_handle(handle)
        return self._settings

    @settings.setter
    def settings(self, value: AudioSettings):
        if not isinstance(value, AudioSettings):
            raise TypeError("settings must be AudioSettings")
        _native.check(_native.lib.oxide_audio_source_set_settings(self._value(), value._value()))
        self._settings = value

    @property
    def state(self) -> PlaybackState:
        """Current playback state; also reports output failures."""
        state = ctypes.c_uint32()
        _native.check(_native.lib.oxide_audio_source_get_state(self._value(), ctypes.byref(state)))
        return PlaybackState(state.value)

    def play(self, audio: Audio) -> None:
        """Replaces the current sound with omnidirectional, distance-attenuated playback."""
        if not isinstance(audio, Audio):
            raise TypeError("audio must be Audio")
        _native.check(_native.lib.oxide_audio_source_play(self._value(), audio._value()))

    def play_directional(self, audio: Audio, inner_half_angle: Angle, outer_half_angle: Angle, outer_volume: float = 0.0) -> None:
        """Replaces playback with cone emission; half-angles range 0..180 degrees.

        Gain fades from one inside the inner cone to outer_volume outside the
        outer cone, with distance attenuation applied as well.
        """
        if not isinstance(audio, Audio):
            raise TypeError("audio must be Audio")
        _native.check(_native.lib.oxide_audio_source_play_directional(self._value(), audio._value(), inner_half_angle, outer_half_angle, outer_volume))

    def set_cone(self, inner_half_angle: Angle, outer_half_angle: Angle, outer_volume: float = 0.0) -> None:
        """Changes the directional cone live; outer_volume must be in [0,1]."""
        _native.check(_native.lib.oxide_audio_source_set_cone(self._value(), inner_half_angle, outer_half_angle, outer_volume))

    def pause(self) -> None:
        """Holds a playing sound at its current cursor; otherwise does nothing."""
        _native.check(_native.lib.oxide_audio_source_pause(self._value()))

    def resume(self) -> None:
        """Resumes paused playback; otherwise does nothing."""
        _native.check(_native.lib.oxide_audio_source_resume(self._value()))

    def stop(self) -> None:
        """Stops playback and releases its clip."""
        _native.check(_native.lib.oxide_audio_source_stop(self._value()))

    @property
    def position(self) -> Vec3:
        """Current world-space position; edits affect playback without restarting."""
        value = Vec3()
        _native.check(_native.lib.oxide_audio_source_get_position(self._value(), ctypes.byref(value)))
        return value

    @position.setter
    def position(self, value: Vec3) -> None:
        _native.check(_native.lib.oxide_audio_source_set_position(self._value(), value))

    @property
    def direction(self) -> Vec3:
        """Current world-space direction; edits affect playback without restarting."""
        value = Vec3()
        _native.check(_native.lib.oxide_audio_source_get_direction(self._value(), ctypes.byref(value)))
        return value

    @direction.setter
    def direction(self, value: Vec3) -> None:
        _native.check(_native.lib.oxide_audio_source_set_direction(self._value(), value))

class AudioListener(_Resource):
    """Independent audio viewpoint; the selected listener is retained natively."""
    _destroy = _native.lib.oxide_audio_listener_destroy

    def __init__(self):
        """Creates an inactive listener at the origin, facing -Z with +Y up."""
        handle = ctypes.c_uint64()
        _native.check(_native.lib.oxide_audio_listener_create(ctypes.byref(handle)))
        self._attach(handle)

    def activate(self) -> None:
        """Selects this listener; later edits apply without reactivation."""
        _native.check(_native.lib.oxide_audio_listener_activate(self._value()))

    def set_orientation(self, direction: Vec3, up: Vec3) -> None:
        """Normalizes and orthogonalizes nonzero, nonparallel forward/up vectors."""
        _native.check(_native.lib.oxide_audio_listener_set_orientation(self._value(), direction, up))

    @property
    def position(self) -> Vec3:
        """Current listener position; orientation vectors are normalized."""
        value = Vec3()
        _native.check(_native.lib.oxide_audio_listener_get_position(self._value(), ctypes.byref(value)))
        return value

    @position.setter
    def position(self, value: Vec3) -> None:
        _native.check(_native.lib.oxide_audio_listener_set_position(self._value(), value))

    @property
    def direction(self) -> Vec3:
        """Current listener direction; orientation vectors are normalized."""
        value = Vec3()
        _native.check(_native.lib.oxide_audio_listener_get_direction(self._value(), ctypes.byref(value)))
        return value

    @property
    def up(self) -> Vec3:
        """Current listener up; orientation vectors are normalized."""
        value = Vec3()
        _native.check(_native.lib.oxide_audio_listener_get_up(self._value(), ctypes.byref(value)))
        return value

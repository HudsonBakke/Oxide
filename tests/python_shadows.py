"""Headless checks for shadow ABI values and the interactive Python example."""
import ctypes
from pathlib import Path
import sys
import unittest
from unittest.mock import Mock, patch

from oxide import DirectionalShadowSettings, GameTime, Vec3, Keys
from oxide import _native
from oxide.math import Rect

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "examples" / "python"))
from shadows import Shadows


class ShadowBindingsTests(unittest.TestCase):
    def test_defaults_match_native_abi(self):
        native_defaults = _native.lib.oxide_directional_shadow_settings_default
        native_defaults.argtypes = [ctypes.POINTER(DirectionalShadowSettings)]
        native_defaults.restype = ctypes.c_int
        actual = DirectionalShadowSettings(size=1, depth=1, resolution=128)
        _native.check(native_defaults(ctypes.byref(actual)))
        expected = DirectionalShadowSettings()
        self.assertEqual(ctypes.sizeof(actual), 32)
        for field, _ in actual._fields_:
            self.assertEqual(getattr(actual, field), getattr(expected, field), field)
        self.assertEqual(actual.center, Vec3())

    def test_single_submissions_capture_transparency_and_interactive_controls(self):
        demo = Shadows()
        pressed = set()
        keys = Mock()
        keys.is_key_pressed.side_effect = lambda key: key in pressed
        with patch("shadows.Keyboard.get_state", return_value=keys), patch.object(demo, "set_window_name"):
            demo.update(GameTime(1.0 / 60.0, 0.0))
            initial_x = demo.transforms[1].position.x
            demo.update(GameTime(1.0 / 60.0, 1.0))
            self.assertNotEqual(initial_x, demo.transforms[1].position.x)
            graphics = Mock(full_rect=Rect(0, 0, 1280, 800))
            demo.draw(graphics)
            graphics.set_directional_shadows.assert_called_once_with(demo.light, demo.settings)
            names = [call[0] for call in graphics.method_calls]
            self.assertLess(names.index("set_directional_shadows"), names.index("draw_mesh"))
            self.assertLess(names.index("bind_camera"), names.index("draw_mesh"))
            draws = graphics.draw_mesh.call_args_list
            self.assertEqual(len(draws), len(demo.transforms))
            self.assertEqual([call.kwargs.get("transparency", 0.0) for call in draws], [0, 0.5, 0, 1, 0.5, 0.5])
            for call, transform in zip(draws, demo.transforms):
                self.assertIs(call.args[0], demo.mesh)
                self.assertIs(call.args[2], transform)
            self.assertNotIn("begin_shadow_pass", names)
            self.assertNotIn("draw_shadow_mesh", names)
            pressed.add(Keys.SPACE)
            demo.update(GameTime(1.0 / 60.0, 2.0))
            self.assertFalse(demo.enabled)
            graphics.reset_mock()
            demo.draw(graphics)
            graphics.set_directional_shadows.assert_not_called()
            graphics.clear_directional_shadows.assert_called_once()
            self.assertEqual(graphics.draw_mesh.call_count, len(demo.transforms))
            pressed.clear()
            pressed.add(Keys.T)
            for expected in [1.0, 0.0, 0.5]:
                demo.update(GameTime(1.0 / 60.0, 3.0))
                self.assertEqual(demo.transparency, expected)


if __name__ == "__main__":
    unittest.main()

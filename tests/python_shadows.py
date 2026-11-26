"""Headless checks for shadow ABI values and the interactive Python example."""
import ctypes
from pathlib import Path
import sys
import unittest
from unittest.mock import Mock, patch

from oxide import DirectionalShadowSettings, GameTime, Vec3
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

    def test_moving_casters_share_transforms_and_space_toggles_the_pass(self):
        demo = Shadows()
        keys = Mock()
        keys.is_key_pressed.return_value = False
        with patch("shadows.Keyboard.get_state", return_value=keys), patch.object(demo, "set_window_name"):
            demo.update(GameTime(1.0 / 60.0, 0.0))
            initial_x = demo.transforms[1].position.x
            demo.update(GameTime(1.0 / 60.0, 1.0))
            self.assertNotEqual(initial_x, demo.transforms[1].position.x)

            graphics = Mock(full_rect=Rect(0, 0, 1280, 800))
            demo.draw(graphics)
            names = [call[0] for call in graphics.method_calls]
            self.assertLess(names.index("end_shadow_pass"), names.index("bind_camera"))
            self.assertLess(names.index("bind_camera"), names.index("draw_mesh"))
            shadow_draws = graphics.draw_shadow_mesh.call_args_list
            scene_draws = graphics.draw_mesh.call_args_list
            self.assertEqual(len(shadow_draws), 3)
            self.assertEqual(len(scene_draws), 3)
            for shadow, scene in zip(shadow_draws, scene_draws):
                self.assertIs(shadow.args[0], scene.args[0])
                self.assertIs(shadow.args[1], scene.args[2])
            self.assertIs(graphics.begin_shadow_pass.call_args.args[0], graphics.add_light.call_args.args[0])

            keys.is_key_pressed.return_value = True
            demo.update(GameTime(1.0 / 60.0, 2.0))
            self.assertFalse(demo.enabled)
            graphics.reset_mock()
            demo.draw(graphics)
            graphics.begin_shadow_pass.assert_not_called()
            self.assertEqual(graphics.draw_mesh.call_count, 3)
            demo.update(GameTime(1.0 / 60.0, 3.0))
            self.assertTrue(demo.enabled)


if __name__ == "__main__":
    unittest.main()

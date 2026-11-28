"""Native Rect3D layout/validation and interactive example control regressions."""
import ctypes
from pathlib import Path
import sys
import unittest
from unittest.mock import Mock, patch
from oxide import Rect3D, Vec3, InvalidArgumentError, Camera, Keys, GameTime
from oxide import _native
from oxide.math import Rect
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/"examples"/"python"))
from monitors import Monitors, monitor

class MonitorBindingsTests(unittest.TestCase):
    def test_rectangle_layout_normalization_and_validation(self):
        self.assertEqual(ctypes.sizeof(Rect3D),60)
        self.assertEqual(Rect3D.normal.offset,48)
        rectangle=Rect3D(Vec3(-2,1,0),Vec3(2,1,0),Vec3(2,-1,0),Vec3(-2,-1,0),Vec3(0,0,4))
        self.assertEqual(rectangle.normal,Vec3(0,0,1))
        self.assertEqual((rectangle.width,rectangle.height,rectangle.aspect_ratio),(4,2,2))
        with self.assertRaises(InvalidArgumentError):
            Rect3D(rectangle.top_left,rectangle.top_left,rectangle.bottom_right,rectangle.bottom_left,rectangle.normal)
        with self.assertRaises(InvalidArgumentError):
            Rect3D(rectangle.top_left,rectangle.top_right,rectangle.bottom_right,rectangle.bottom_left,Vec3(1,0,0))
        self.assertEqual(_native.lib.oxide_graphics_bind_camera_3d(_native.NativeGraphicsHandle(),_native.NativeCameraHandle(),rectangle),1)
        rectangle.normal=Vec3()
        self.assertEqual(_native.lib.oxide_graphics_bind_camera_3d(_native.NativeGraphicsHandle(),_native.NativeCameraHandle(),rectangle),1)

    def test_example_updates_controls_and_submits_both_orders(self):
        demo=Monitors()
        pressed=set()
        keyboard=Mock()
        keyboard.is_key_pressed.side_effect=lambda key:key in pressed
        with patch("monitors.Keyboard.get_state",return_value=keyboard),patch.object(demo,"set_window_name"),patch.object(demo,"exit") as exit_game:
            demo.update(GameTime(1/60,0))
            first=demo.occluder.position.x
            demo.update(GameTime(1/60,1))
            self.assertNotEqual(first,demo.occluder.position.x)
            graphics=Mock(full_rect=Rect(0,0,1280,720))
            demo.draw(graphics)
            names=[call[0] for call in graphics.method_calls]
            self.assertLess(names.index("bind_camera_3d"),names.index("bind_camera"))
            self.assertEqual(graphics.bind_camera_3d.call_count,2)
            pressed.add(Keys.SPACE)
            demo.update(GameTime(1/60,2))
            graphics.reset_mock();demo.draw(graphics)
            names=[call[0] for call in graphics.method_calls]
            self.assertLess(names.index("bind_camera"),names.index("bind_camera_3d"))
            pressed.clear();pressed.add(Keys.T)
            for expected in [1.0,0.0,0.5]:
                demo.update(GameTime(1/60,3));self.assertEqual(demo.transparency,expected)
            pressed.clear();pressed.add(Keys.B)
            demo.update(GameTime(1/60,3))
            graphics.reset_mock();demo.draw(graphics)
            self.assertEqual(graphics.bind_camera_3d.call_args.args[1].normal,Vec3(0,0,-1))
            pressed.clear();pressed.add(Keys.ESCAPE)
            demo.update(GameTime(1/60,3));exit_game.assert_called_once()

if __name__=="__main__":
    unittest.main()

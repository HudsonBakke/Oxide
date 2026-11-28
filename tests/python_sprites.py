"""2D ABI, snapshot/UTF-8 marshalling, lifetime guards and menu input checks."""
import ctypes
from pathlib import Path
import sys
import unittest
from unittest.mock import Mock, patch
from oxide import (Draw2DOptions, TextOptions, Color, Rect, Vec2, Texture, Graphics, GameTime, Keys, MouseButtons)
from oxide import _native
from oxide.graphics import _NativeDraw2DOptions
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'examples' / 'python'))
from menus import Menus


class SpriteChecks(unittest.TestCase):
    def test_layout_and_independent_defaults(self):
        self.assertEqual(ctypes.sizeof(_NativeDraw2DOptions), 64)
        self.assertEqual(_NativeDraw2DOptions.flags.offset, 60)
        a, b = Draw2DOptions(), Draw2DOptions()
        a.tint.a = 0.5
        self.assertEqual(b.tint.a, 1)
        options = Draw2DOptions(source=Rect(1,0,1,1), clip=Rect(2,3,4,5), flip_horizontally=True)
        snapshot = options._snapshot()
        self.assertEqual(snapshot.flags, 7)
        options.source.x = 9
        self.assertEqual(snapshot.source.x, 1)
        # Real native boundary rejects malformed fields and null graphics safely.
        native = _native.lib.oxide_graphics_draw_2d_with_options
        self.assertEqual(native(_native.NativeGraphicsHandle(), _native.NativeTextureHandle(), Rect(0,0,1,1), b._snapshot()), 1)
        self.assertEqual(native(_native.NativeGraphicsHandle(), _native.NativeTextureHandle(), Rect(0,0,-1,1), b._snapshot()), 1)

    def test_text_utf8_capture_and_callback_guard(self):
        graphics = Graphics(_native.NativeGraphicsHandle())
        options = TextOptions(tint=Color(0,1,0), font_size=32)
        with patch.object(_native.lib, 'oxide_graphics_draw_text', return_value=0) as draw:
            graphics.draw_text('café 日本語', Vec2(1,2), Color(1,0,0), options)
            args = draw.call_args.args
            self.assertEqual(args[1], 'café 日本語'.encode())
            self.assertEqual(args[2], len(args[1]))
            self.assertEqual(args[-1].tint, Color(1,0,0))
            self.assertEqual(options.tint, Color(0,1,0))
        graphics._invalidate()
        with self.assertRaises(RuntimeError): graphics.draw_text('label', Vec2())
        with self.assertRaises(RuntimeError): graphics.measure_text('label')

    def test_menu_controls_and_draws_stay_connected(self):
        demo = Menus()
        pressed = set()
        keys = Mock(); keys.is_key_pressed.side_effect = lambda key: key in pressed
        mouse = Mock(position=Vec2(-1,-1), delta=Vec2())
        mouse.is_button_pressed.return_value = False
        with patch('menus.Keyboard.get_state', return_value=keys), patch('menus.Mouse.get_state', return_value=mouse), patch.object(demo, 'exit') as exit_game:
            demo.update(GameTime(0.1,1)); self.assertGreater(demo.angle,0)
            pressed.add(Keys.ENTER); demo.update(GameTime(0.1,2)); self.assertTrue(demo.paused)
            pressed.clear(); pressed.add(Keys.DOWN); demo.update(GameTime(0.1,3)); self.assertEqual(demo.selected,1)
            pressed.clear(); pressed.add(Keys.ENTER); demo.update(GameTime(0.1,4)); self.assertEqual(demo.exposure,2)
            pressed.clear(); pressed.update([Keys.F,Keys.C]); demo.update(GameTime(0.1,5))
            self.assertTrue(demo.flipped); self.assertFalse(demo.clipped)
            pressed.clear(); mouse.position = Vec2(50,275); mouse.is_button_pressed.return_value = True
            demo.update(GameTime(0.1,6)); exit_game.assert_called_once()
        graphics = Mock(full_rect=Rect(0,0,900,600))
        graphics.measure_text.return_value=Vec2(50,24)
        demo.draw(graphics)
        names = [call[0] for call in graphics.method_calls]
        self.assertLess(names.index('draw_2d'), names.index('draw_mesh'))
        self.assertEqual(graphics.draw_mesh.call_count, 1)
        self.assertGreaterEqual(graphics.draw_text.call_count, 6)


if __name__ == '__main__': unittest.main()

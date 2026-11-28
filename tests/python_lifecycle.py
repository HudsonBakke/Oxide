"""Callback exception identity/tracebacks, prompt exit, and preflight validation."""
import ctypes
import traceback
import unittest
from unittest.mock import patch
from oxide import AppConfig, Game, Graphics, GraphicsError
from oxide import _native


class CallbackFailure(BaseException):
    pass


class LifecycleTests(unittest.TestCase):
    def test_callbacks_request_exit_keep_first_exception_and_invalidate_graphics(self):
        for phase in ('initialize', 'update', 'draw'):
            with self.subTest(phase=phase):
                original = CallbackFailure(phase)
                calls, retained = [], []
                class BrokenGame(Game):
                    def initialize(self):
                        calls.append('initialize')
                        if phase == 'initialize': raise original
                    def update(self, time):
                        calls.append('update')
                        if phase == 'update': raise original
                    def draw(self, graphics):
                        retained.append(graphics)
                        calls.append('draw')
                        if phase == 'draw': raise original
                game = BrokenGame()
                with patch.object(_native.lib, 'oxide_app_exit', return_value=0) as exit_request:
                    def native_run(config, callbacks):
                        callbacks.initialize(None)
                        if not exit_request.called:
                            callbacks.update(None, _native.NativeGameTime(0.1, 0.1))
                        if not exit_request.called:
                            callbacks.draw(None, _native.NativeGraphicsHandle())
                        # Even spurious callbacks after the failure must be suppressed.
                        callbacks.initialize(None)
                        callbacks.update(None, _native.NativeGameTime(0.1, 0.2))
                        callbacks.draw(None, _native.NativeGraphicsHandle())
                        return 4  # A later native failure cannot hide the original exception.
                    with patch.object(_native.lib, 'oxide_run', side_effect=native_run):
                        try:
                            game.run()
                        except CallbackFailure as caught:
                            self.assertIs(caught, original)
                            self.assertIn(phase, [f.name for f in traceback.extract_tb(caught.__traceback__)])
                        else:
                            self.fail('callback failure was lost')
                    exit_request.assert_called_once()
                self.assertEqual(calls, ['initialize', 'update', 'draw'][:['initialize', 'update', 'draw'].index(phase)+1])
                for graphics in retained:
                    with self.assertRaises(RuntimeError): graphics._require_valid()

    def test_clean_exit_and_configuration_errors(self):
        game = Game()
        with patch.object(_native.lib, 'oxide_run', return_value=0) as run:
            for config in [AppConfig(width=0), AppConfig(height=0), AppConfig(title='bad\0title')]:
                with self.assertRaises(ValueError): game.run(config)
            run.assert_not_called()
            game.run()
            run.assert_called_once()
            with self.assertRaises(RuntimeError): game.run()
        with patch.object(_native.lib, 'oxide_run', return_value=4), patch.object(_native.lib, 'oxide_last_error_message', return_value=b'failed to initialize Oxide graphics: unavailable adapter'):
            with self.assertRaisesRegex(GraphicsError, 'unavailable adapter'): Game().run()


if __name__ == '__main__': unittest.main()

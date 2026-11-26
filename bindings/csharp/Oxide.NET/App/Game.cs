using System;
using System.Runtime.ExceptionServices;
using System.Runtime.InteropServices;

using Oxide.Native;
using Oxide.Graphics;

namespace Oxide.App;

public abstract class Game
    : IDisposable
{
    private ExceptionDispatchInfo?
        _callbackException;

    private bool
        _disposed;

    protected virtual void Initialize()
    {
    }

    protected virtual void Update(
        GameTime time
    )
    {
    }

    protected virtual void Draw(
        Graphics.Graphics graphics
    )
    {
    }

    public uint WindowWidth =>
        NativeMethods.oxide_app_window_width();

    public uint WindowHeight =>
        NativeMethods.oxide_app_window_height();

    public bool IsWindowFocused =>
        NativeMethods.oxide_app_is_window_focused() != 0;

    public bool IsFullscreen
    {
        get =>
            NativeMethods.oxide_app_is_fullscreen() != 0;

        set =>
            NativeMethods.Check(
                NativeMethods.oxide_app_set_fullscreen(
                    value ? (byte)1 : (byte)0
                )
            );
    }

    public void Exit()
    {
        NativeMethods.Check(
            NativeMethods.oxide_app_exit()
        );
    }

    public void SetWindowName(
        string name
    )
    {
        ArgumentNullException.ThrowIfNull(name);

        IntPtr nativeName =
            Marshal.StringToCoTaskMemUTF8(name);

        try
        {
            NativeMethods.Check(
                NativeMethods.oxide_app_set_window_name(
                    nativeName
                )
            );
        }
        finally
        {
            Marshal.FreeCoTaskMem(nativeName);
        }
    }

    public void SetWindowLogo(
        Texture texture
    )
    {
        ArgumentNullException.ThrowIfNull(texture);

        NativeMethods.Check(
            NativeMethods.oxide_app_set_window_logo(
                texture.Handle
            )
        );
    }

    public void SetPreferredBackbufferSize(
        uint width,
        uint height
    )
    {
        if (width == 0)
        {
            throw new ArgumentOutOfRangeException(nameof(width));
        }

        if (height == 0)
        {
            throw new ArgumentOutOfRangeException(nameof(height));
        }

        NativeMethods.Check(
            NativeMethods.oxide_app_set_preferred_backbuffer_size(
                width,
                height
            )
        );
    }

    public void SetPreferredBackbufferWidth(
        uint width
    )
    {
        SetPreferredBackbufferSize(
            width,
            WindowHeight
        );
    }

    public void SetPreferredBackbufferHeight(
        uint height
    )
    {
        SetPreferredBackbufferSize(
            WindowWidth,
            height
        );
    }

    public void SetCursorVisible(
        bool visible
    )
    {
        NativeMethods.Check(
            NativeMethods.oxide_app_set_cursor_visible(
                visible ? (byte)1 : (byte)0
            )
        );
    }

    public void SetCursorGrabbed(
        bool grabbed
    )
    {
        NativeMethods.Check(
            NativeMethods.oxide_app_set_cursor_grabbed(
                grabbed ? (byte)1 : (byte)0
            )
        );
    }

    public void Run()
    {
        Run(
            new AppConfig()
        );
    }

    public void Run(
        AppConfig config
    )
    {
        ObjectDisposedException.ThrowIf(
            _disposed,
            this
        );

        ArgumentNullException.ThrowIfNull(
            config
        );

        // A Game instance represents one application
        // lifetime. Running it twice would attempt to start
        // the native event loop again.
        if (_hasRun)
        {
            throw new InvalidOperationException(
                "An Oxide Game instance can only be run once."
            );
        }

        _hasRun = true;

        IntPtr title =
            Marshal.StringToCoTaskMemUTF8(
                config.Title
            );

        NativeInitializeCallback initialize =
            OnInitialize;

        NativeUpdateCallback update =
            OnUpdate;

        NativeDrawCallback draw =
            OnDraw;

        try
        {
            var nativeConfig =
                new NativeAppConfig {
                    Title = title,
                    Width = config.Width,
                    Height = config.Height,
                };

            var callbacks =
                new NativeGameCallbacks {
                    UserData =
                        IntPtr.Zero,

                    Initialize =
                        initialize,

                    Update =
                        update,

                    Draw =
                        draw,
                };

            OxideStatus status =
                NativeMethods.oxide_run(
                    nativeConfig,
                    callbacks
                );

            // Native code stores these function pointers for
            // the entire application lifetime.
            GC.KeepAlive(initialize);
            GC.KeepAlive(update);
            GC.KeepAlive(draw);

            // Managed exceptions are caught inside callbacks
            // so they never cross the native ABI. Rethrow
            // safely once native execution has returned.
            _callbackException?
                .Throw();

            NativeMethods.Check(
                status
            );
        }
        finally
        {
            Marshal.FreeCoTaskMem(
                title
            );
        }
    }

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }

        Dispose(
            true
        );

        _disposed = true;

        GC.SuppressFinalize(
            this
        );
    }

    /// Framework-level cleanup hook.
    ///
    /// Game subclasses normally do not need to override this.
    protected virtual void Dispose(
        bool disposing
    )
    {
    }

    private bool
        _hasRun;

    private void OnInitialize(
        IntPtr userData
    )
    {
        if (_callbackException
            is not null)
        {
            return;
        }

        try
        {
            Initialize();
        }
        catch (Exception exception)
        {
            CaptureCallbackException(
                exception
            );
        }
    }

    private void OnUpdate(
        IntPtr userData,
        NativeGameTime time
    )
    {
        if (_callbackException
            is not null)
        {
            return;
        }

        try
        {
            Update(
                new GameTime(
                    time.DeltaSeconds,
                    time.TotalSeconds
                )
            );
        }
        catch (Exception exception)
        {
            CaptureCallbackException(
                exception
            );
        }
    }

    private void OnDraw(
        IntPtr userData,
        NativeGraphicsHandle graphicsHandle
    )
    {
        if (_callbackException
            is not null)
        {
            return;
        }

        var graphics =
            new Graphics.Graphics(
                graphicsHandle
            );

        try
        {
            Draw(
                graphics
            );
        }
        catch (Exception exception)
        {
            CaptureCallbackException(
                exception
            );
        }
        finally
        {
            graphics.Invalidate();
        }
    }

    private void CaptureCallbackException(
        Exception exception
    )
    {
        _callbackException ??=
            ExceptionDispatchInfo.Capture(
                exception
            );
    }
}
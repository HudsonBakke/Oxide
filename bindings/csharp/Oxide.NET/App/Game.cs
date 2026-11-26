using System;
using System.Runtime.ExceptionServices;
using System.Runtime.InteropServices;

using Oxide.Native;
using Oxide.Graphics;

namespace Oxide.App;

/// <summary>Base class for an Oxide application. Override lifecycle methods to initialize, update, and draw the game.</summary>
public abstract class Game
    : IDisposable
{
    private ExceptionDispatchInfo?
        _callbackException;

    private bool
        _disposed;

    private bool
        _hasRun;

        /// <summary>Called once after Oxide has initialized the application.</summary>
protected virtual void Initialize()
    {
    }

    /// <summary>Called once per update with timing information.</summary>
    protected virtual void Update(
        GameTime time
    )
    {
    }

        /// <summary>Called each frame to submit rendering commands.</summary>
protected virtual void Draw(
        Graphics.Graphics graphics
    )
    {
    }

    /// <summary>Current window width in pixels.</summary>
    public uint WindowWidth =>
        NativeMethods.oxide_app_window_width();

    /// <summary>Current window height in pixels.</summary>
    public uint WindowHeight =>
        NativeMethods.oxide_app_window_height();

    /// <summary>Whether the Oxide window currently has input focus.</summary>
    public bool IsWindowFocused =>
        NativeMethods.oxide_app_is_window_focused() != 0;

    /// <summary>Gets or sets fullscreen mode.</summary>
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

        /// <summary>Requests that the running application exit.</summary>
public void Exit()
    {
        NativeMethods.Check(
            NativeMethods.oxide_app_exit()
        );
    }

        /// <summary>Changes the native window title.</summary>
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

        /// <summary>Changes the native window icon using the supplied texture.</summary>
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

        /// <summary>Requests a new backbuffer width and height.</summary>
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

    /// <summary>Requests a new backbuffer width while preserving height.</summary>
    public void SetPreferredBackbufferWidth(
        uint width
    )
    {
        SetPreferredBackbufferSize(
            width,
            WindowHeight
        );
    }

    /// <summary>Requests a new backbuffer height while preserving width.</summary>
    public void SetPreferredBackbufferHeight(
        uint height
    )
    {
        SetPreferredBackbufferSize(
            WindowWidth,
            height
        );
    }

        /// <summary>Shows or hides the system cursor while the application is running.</summary>
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

        /// <summary>Controls whether the cursor is confined to the Oxide window.</summary>
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

        /// <summary>Starts the Oxide application loop. A Game instance can be run only once.</summary>
public void Run()
    {
        Run(
            new AppConfig()
        );
    }

    /// <summary>Starts the application loop; a Game instance can run only once.</summary>
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

    /// <summary>Releases managed resources associated with the game instance.</summary>
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
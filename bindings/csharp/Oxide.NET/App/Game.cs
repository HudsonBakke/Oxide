using System;
using System.Runtime.ExceptionServices;
using System.Runtime.InteropServices;

using Oxide.Native;
using Oxide.Graphics;

namespace Oxide.App;

/// <summary>Application lifecycle implemented by initialize, update, and draw callbacks.</summary>
public abstract class Game : IDisposable
{
    private ExceptionDispatchInfo? _callbackException;

    private bool _disposed;

    private bool _hasRun;

    /// <summary>Called once after the window and graphics are ready; load persistent resources here.</summary>
    protected virtual void Initialize() {}

    /// <summary>Called once before drawing each frame with elapsed time in seconds; update game logic and read input here.</summary>
    protected virtual void Update(GameTime time) {}

    /// <summary>Called each frame to bind cameras, set lighting, and submit draw calls.</summary>
    protected virtual void Draw(Graphics.Graphics graphics) {}

    /// <summary>Current client-area width in physical pixels.</summary>
    public uint WindowWidth => NativeMethods.oxide_app_window_width();
    /// <summary>Current client-area height in physical pixels.</summary>
    public uint WindowHeight => NativeMethods.oxide_app_window_height();
    /// <summary>Whether the application window currently has keyboard focus.</summary>
    public bool IsWindowFocused => NativeMethods.oxide_app_is_window_focused() != 0;
    /// <summary>Whether borderless fullscreen is enabled or has been requested.</summary>
    public bool IsFullscreen
    {
        get => NativeMethods.oxide_app_is_fullscreen() != 0;

        set =>
            NativeMethods.Check(NativeMethods.oxide_app_set_fullscreen(value ? (byte)1 : (byte)0));
    }

    /// <summary>Queues a request to leave the application event loop.</summary>
    public void Exit()
    {
        NativeMethods.Check(NativeMethods.oxide_app_exit());
    }

    /// <summary>Queues a change to the native window title.</summary>
    public void SetWindowName(string name)
    {
        ArgumentNullException.ThrowIfNull(name);

        IntPtr nativeName = Marshal.StringToCoTaskMemUTF8(name);

        try
        {
            NativeMethods.Check(NativeMethods.oxide_app_set_window_name(nativeName));
        }
        finally
        {
            Marshal.FreeCoTaskMem(nativeName);
        }
    }

    /// <summary>Queues an icon change using an RGBA8 texture.</summary>
    public void SetWindowLogo(Texture texture)
    {
        ArgumentNullException.ThrowIfNull(texture);

        NativeMethods.Check(NativeMethods.oxide_app_set_window_logo(texture.Handle));
    }

    /// <summary>Requests a client/backbuffer size in physical pixels; both dimensions must be positive.</summary>
    public void SetPreferredBackbufferSize(uint width, uint height)
    {
        if (width == 0)
        {
            throw new ArgumentOutOfRangeException(nameof(width));
        }

        if (height == 0)
        {
            throw new ArgumentOutOfRangeException(nameof(height));
        }

        NativeMethods.Check(NativeMethods.oxide_app_set_preferred_backbuffer_size(width, height));
    }

    /// <summary>Requests a physical-pixel width while retaining the current window height.</summary>
    public void SetPreferredBackbufferWidth(uint width)
    {
        SetPreferredBackbufferSize(width, WindowHeight);
    }

    /// <summary>Requests a physical-pixel height while retaining the current window width.</summary>
    public void SetPreferredBackbufferHeight(uint height)
    {
        SetPreferredBackbufferSize(WindowWidth, height);
    }

    /// <summary>Queues a change to the visibility of the OS cursor over the game window.</summary>
    public void SetCursorVisible(bool visible)
    {
        NativeMethods.Check(
            NativeMethods.oxide_app_set_cursor_visible(visible ? (byte)1 : (byte)0));
    }

    /// <summary>Requests cursor locking, falling back to confinement when locking is unavailable.</summary>
    public void SetCursorGrabbed(bool grabbed)
    {
        NativeMethods.Check(
            NativeMethods.oxide_app_set_cursor_grabbed(grabbed ? (byte)1 : (byte)0));
    }

    /// <summary>Runs the blocking application event loop on the calling thread until the window closes or exit is requested.</summary>
    public void Run()
    {
        Run(new AppConfig());
    }

    /// <summary>Runs the blocking application event loop on the calling thread until the window closes or exit is requested.</summary>
    public void Run(AppConfig config)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);

        ArgumentNullException.ThrowIfNull(config);

        // A Game instance represents one application
        // lifetime. Running it twice would attempt to start
        // the native event loop again.
        if (_hasRun)
        {
            throw new InvalidOperationException("An Oxide Game instance can only be run once.");
        }

        _hasRun = true;

        IntPtr title = Marshal.StringToCoTaskMemUTF8(config.Title);

        NativeInitializeCallback initialize = OnInitialize;

        NativeUpdateCallback update = OnUpdate;

        NativeDrawCallback draw = OnDraw;

        try
        {
            var nativeConfig = new NativeAppConfig {
                Title = title,
                Width = config.Width,
                Height = config.Height,
            };

            var callbacks = new NativeGameCallbacks {
                UserData = IntPtr.Zero,

                Initialize = initialize,

                Update = update,

                Draw = draw,
            };

            OxideStatus status = NativeMethods.oxide_run(nativeConfig, callbacks);

            // Native code stores these function pointers for
            // the entire application lifetime.
            GC.KeepAlive(initialize);
            GC.KeepAlive(update);
            GC.KeepAlive(draw);

            // Managed exceptions are caught inside callbacks
            // so they never cross the native ABI. Rethrow
            // safely once native execution has returned.
            _callbackException ?.Throw();

            NativeMethods.Check(status);
        }
        finally
        {
            Marshal.FreeCoTaskMem(title);
        }
    }

    /// <summary>Releases the callback resources owned by this game; overriding implementations should call the base method.</summary>
    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }

        Dispose(true);

        _disposed = true;

        GC.SuppressFinalize(this);
    }

    /// <summary>Releases the callback resources owned by this game; overriding implementations should call the base method.</summary>
    protected virtual void Dispose(bool disposing) {}

    private void OnInitialize(IntPtr userData)
    {
        if (_callbackException is not null)
        {
            return;
        }

        try
        {
            Initialize();
        }
        catch (Exception exception)
        {
            CaptureCallbackException(exception);
        }
    }

    private void OnUpdate(IntPtr userData, NativeGameTime time)
    {
        if (_callbackException is not null)
        {
            return;
        }

        try
        {
            Update(new GameTime(time.DeltaSeconds, time.TotalSeconds));
        }
        catch (Exception exception)
        {
            CaptureCallbackException(exception);
        }
    }

    private void OnDraw(IntPtr userData, NativeGraphicsHandle graphicsHandle)
    {
        if (_callbackException is not null)
        {
            return;
        }

        var graphics = new Graphics.Graphics(graphicsHandle);

        try
        {
            Draw(graphics);
        }
        catch (Exception exception)
        {
            CaptureCallbackException(exception);
        }
        finally
        {
            graphics.Invalidate();
        }
    }

    private void CaptureCallbackException(Exception exception)
    {
        _callbackException ??= ExceptionDispatchInfo.Capture(exception);
    }
}

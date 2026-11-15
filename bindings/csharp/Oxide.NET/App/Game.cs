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
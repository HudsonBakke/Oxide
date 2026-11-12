using System;
using System.Runtime.ExceptionServices;
using System.Runtime.InteropServices;

using Oxide.Native;

namespace Oxide;

public abstract class Game
{
    private ExceptionDispatchInfo?
        _callbackException;

    protected virtual void Initialize()
    {
    }

    protected virtual void Update(
        GameTime time
    )
    {
    }

    protected virtual void Draw(
        Graphics graphics
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
        ArgumentNullException.ThrowIfNull(
            config
        );

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

            // Keep the managed delegate objects alive until
            // native execution has completely returned.
            GC.KeepAlive(initialize);
            GC.KeepAlive(update);
            GC.KeepAlive(draw);

            // A managed exception must never propagate through
            // an unmanaged callback frame. The callback catches
            // it, and we rethrow it here once Rust has returned.
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

    private void OnInitialize(
        IntPtr userData
    )
    {
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
            new Graphics(
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
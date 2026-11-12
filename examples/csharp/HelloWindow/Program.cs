using System;
using System.Runtime.InteropServices;

internal static class Program
{
    private enum OxideStatus
    {
        Ok = 0,
        InvalidArgument = 1,
        InvalidOperation = 2,
        ContentError = 3,
        GraphicsError = 4,
        InternalError = 5,
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct OxideGameTime
    {
        public double DeltaSeconds;
        public double TotalSeconds;
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct OxideGraphicsHandle
    {
        public IntPtr Ptr;
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct OxideAppConfig
    {
        public IntPtr Title;
        public uint Width;
        public uint Height;
    }

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    private delegate void InitializeCallback(
        IntPtr userData
    );

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    private delegate void UpdateCallback(
        IntPtr userData,
        OxideGameTime time
    );

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    private delegate void DrawCallback(
        IntPtr userData,
        OxideGraphicsHandle graphics
    );

    [StructLayout(LayoutKind.Sequential)]
    private struct OxideGameCallbacks
    {
        public IntPtr UserData;

        public InitializeCallback Initialize;
        public UpdateCallback Update;
        public DrawCallback Draw;
    }

    private static class Native
    {
        [DllImport(
            "oxide",
            CallingConvention =
                CallingConvention.Cdecl
        )]
        public static extern OxideStatus oxide_run(
            OxideAppConfig config,
            OxideGameCallbacks callbacks
        );

        [DllImport(
            "oxide",
            CallingConvention =
                CallingConvention.Cdecl
        )]
        public static extern IntPtr
            oxide_last_error_message();
    }

    private static void Check(
        OxideStatus status
    )
    {
        if (status == OxideStatus.Ok)
        {
            return;
        }

        IntPtr messagePtr =
            Native.oxide_last_error_message();

        string message =
            messagePtr == IntPtr.Zero
                ? $"Oxide failed with status {status}."
                : Marshal.PtrToStringUTF8(
                    messagePtr
                )
                ?? $"Oxide failed with status {status}.";

        throw new InvalidOperationException(
            message
        );
    }

    private static void Initialize(
        IntPtr userData
    )
    {
        Console.WriteLine(
            "C# initialize callback!"
        );
    }

    private static void Update(
        IntPtr userData,
        OxideGameTime time
    )
    {
        if (!_printedUpdate)
        {
            Console.WriteLine(
                $"C# update callback! " +
                $"delta = {time.DeltaSeconds:F6}, " +
                $"total = {time.TotalSeconds:F6}"
            );

            _printedUpdate = true;
        }
    }

    private static void Draw(
        IntPtr userData,
        OxideGraphicsHandle graphics
    )
    {
        if (!_printedDraw)
        {
            Console.WriteLine(
                "C# draw callback!"
            );

            _printedDraw = true;
        }
    }

    private static bool _printedUpdate;
    private static bool _printedDraw;

    private static void Main()
    {
        IntPtr title =
            Marshal.StringToCoTaskMemUTF8(
                "Oxide C# ABI Test"
            );

        try
        {
            var initialize =
                new InitializeCallback(
                    Initialize
                );

            var update =
                new UpdateCallback(
                    Update
                );

            var draw =
                new DrawCallback(
                    Draw
                );

            var config =
                new OxideAppConfig {
                    Title = title,
                    Width = 1280,
                    Height = 720,
                };

            var callbacks =
                new OxideGameCallbacks {
                    UserData =
                        IntPtr.Zero,

                    Initialize =
                        initialize,

                    Update =
                        update,

                    Draw =
                        draw,
                };

            Console.WriteLine(
                "Starting Oxide from C#..."
            );

            Check(
                Native.oxide_run(
                    config,
                    callbacks
                )
            );

            // Explicitly keep the delegates alive for the
            // entire native event loop.
            GC.KeepAlive(initialize);
            GC.KeepAlive(update);
            GC.KeepAlive(draw);

            Console.WriteLine(
                "Oxide returned to C#."
            );
        }
        finally
        {
            Marshal.FreeCoTaskMem(
                title
            );
        }
    }
}
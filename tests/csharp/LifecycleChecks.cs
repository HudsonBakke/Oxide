using System.Reflection;
using System.Runtime.ExceptionServices;
using Oxide.App;
using Oxide.Graphics;

internal static class LifecycleChecks
{
    private sealed class CallbackFailure : Exception { }
    private sealed class BrokenGame(string phase) : Game
    {
        internal readonly CallbackFailure Original = new();
        internal readonly List<string> Calls = [];
        internal Graphics? Retained;
        private void OriginalThrow() => throw Original;
        protected override void Initialize() { Calls.Add("initialize"); if (phase=="initialize") OriginalThrow(); }
        protected override void Update(GameTime time) { Calls.Add("update"); if (phase=="update") OriginalThrow(); }
        protected override void Draw(Graphics graphics) { Retained=graphics; Calls.Add("draw"); if (phase=="draw") OriginalThrow(); }
    }
    internal static void Run()
    {
        var flags=BindingFlags.Instance|BindingFlags.NonPublic;
        var assembly=typeof(Game).Assembly;
        var nativeTime=Activator.CreateInstance(assembly.GetType("Oxide.Native.NativeGameTime")!)!;
        var nativeGraphics=Activator.CreateInstance(assembly.GetType("Oxide.Native.NativeGraphicsHandle")!)!;
        foreach (var phase in new[] {"initialize","update","draw"})
        {
            using var game=new BrokenGame(phase);
            void Initialize() => typeof(Game).GetMethod("OnInitialize",flags)!.Invoke(game,[IntPtr.Zero]);
            void Update() => typeof(Game).GetMethod("OnUpdate",flags)!.Invoke(game,[IntPtr.Zero,nativeTime]);
            void Draw() => typeof(Game).GetMethod("OnDraw",flags)!.Invoke(game,[IntPtr.Zero,nativeGraphics]);
            Initialize(); Update(); Draw(); Initialize(); Update(); Draw();
            int expected=phase=="initialize" ? 1 : phase=="update" ? 2 : 3;
            if (game.Calls.Count!=expected) throw new Exception("Callbacks continued after failure");
            var captured=(ExceptionDispatchInfo)typeof(Game).GetField("_callbackException",flags)!.GetValue(game)!;
            try { captured.Throw(); }
            catch (CallbackFailure failure)
            {
                if (!ReferenceEquals(failure,game.Original) || !failure.StackTrace!.Contains("OriginalThrow")) throw new Exception("Callback exception identity/stack was lost");
            }
            if (game.Retained is not null)
            {
                bool invalid=false;
                try { _=game.Retained.FullRect; } catch (InvalidOperationException) { invalid=true; }
                if (!invalid) throw new Exception("Failed draw retained a live Graphics handle");
            }
        }
        using var invalidGame=new BrokenGame("none");
        try { invalidGame.Run(new AppConfig {Width=0}); throw new Exception("Invalid config was accepted"); }
        catch (ArgumentException) { }
        Console.WriteLine("Application callback lifecycle checks passed");
    }
}

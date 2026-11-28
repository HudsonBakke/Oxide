using System.Runtime.InteropServices;
using Oxide.Math;

namespace Oxide.Graphics;

/// <summary>Captured screen-space sprite effects. Coordinates use physical pixels.</summary>
public class Draw2DOptions
{
    /// <summary>Linear RGBA multiplier in [0,1]; alpha controls opacity. Defaults to white.</summary>
    public Color Tint { get; set; } = Color.White;
    /// <summary>Texture pixel region, or null for the entire image. Must fit inside the texture.</summary>
    public Rect? SourceRectangle { get; set; }
    /// <summary>Clockwise screen rotation, initially zero.</summary>
    public Angle Rotation { get; set; } = Angle.Radians(0);
    /// <summary>Rotation pivot relative to target top-left, in target pixels. Does not shift an unrotated sprite.</summary>
    public Vec2 Origin { get; set; }
    /// <summary>Screen pixel clip rectangle, or null for the window. Empty clips draw nothing.</summary>
    public Rect? ClipRectangle { get; set; }
    /// <summary>Mirrors sampled UV coordinates horizontally.</summary>
    public bool FlipHorizontally { get; set; }
    /// <summary>Mirrors sampled UV coordinates vertically.</summary>
    public bool FlipVertically { get; set; }
    internal NativeDraw2DOptions Snapshot() => new() {
        Tint = Tint, Source = SourceRectangle ?? default, Rotation = Rotation, Origin = Origin,
        Clip = ClipRectangle ?? default,
        Flags = (SourceRectangle.HasValue ? 1u : 0u) | (ClipRectangle.HasValue ? 2u : 0u) | (FlipHorizontally ? 4u : 0u) | (FlipVertically ? 8u : 0u)
    };
}

/// <summary>Installed-font menu label controls. Labels support Unicode and explicit newlines.</summary>
public sealed class TextOptions : Draw2DOptions
{
    /// <summary>Installed Windows font family. Defaults to Segoe UI; missing glyphs use system fallback.</summary>
    public string FontFamily { get; set; } = "Segoe UI";
    /// <summary>Font em size in physical pixels, from 1 through 256. Defaults to 24.</summary>
    public float FontSize { get; set; } = 24;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeDraw2DOptions
{
    internal Color Tint;
    internal Rect Source;
    internal Angle Rotation;
    internal Vec2 Origin;
    internal Rect Clip;
    internal uint Flags;
}

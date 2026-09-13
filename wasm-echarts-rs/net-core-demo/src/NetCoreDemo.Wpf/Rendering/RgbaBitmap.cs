using System.Windows;
using System.Windows.Media;
using System.Windows.Media.Imaging;

namespace NetCoreDemo.Wpf.Rendering;

internal static class RgbaBitmap
{
    public static WriteableBitmap FromRgba(byte[] rgba, int width, int height)
    {
        var expected = width * height * 4;
        if (rgba.Length < expected)
        {
            throw new ArgumentException($"RGBA buffer is {rgba.Length}, expected at least {expected}");
        }

        var pbgra = new byte[expected];
        for (var i = 0; i < expected; i += 4)
        {
            var r = rgba[i];
            var g = rgba[i + 1];
            var b = rgba[i + 2];
            var a = rgba[i + 3];
            pbgra[i] = (byte)(b * a / 255);
            pbgra[i + 1] = (byte)(g * a / 255);
            pbgra[i + 2] = (byte)(r * a / 255);
            pbgra[i + 3] = a;
        }

        var bitmap = new WriteableBitmap(width, height, 96, 96, PixelFormats.Pbgra32, null);
        bitmap.WritePixels(new Int32Rect(0, 0, width, height), pbgra, width * 4, 0);
        bitmap.Freeze();
        return bitmap;
    }
}

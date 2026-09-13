using System.IO;
using NetCoreDemo.Wpf.Catalog;
using NetCoreDemo.Wpf.Demos;
using WasmCharts.Runtime;

namespace NetCoreDemo.Wpf;

internal static class GallerySmoke
{
    public static int Run()
    {
        var artifacts = Path.GetFullPath(Path.Combine(FindDemoRoot(), "artifacts"));
        Directory.CreateDirectory(artifacts);

        using var echarts = WasmEchartsModule.Load();
        using var zrender = WasmZrenderModule.Load();
        Render(echarts, zrender, DemoCatalog.DefaultZrender, 640, 400, Path.Combine(artifacts, "p1-zrender-hello_world.bmp"));
        Render(echarts, zrender, DemoCatalog.Echarts.First(item => item.Id == "line"), 480, 360, Path.Combine(artifacts, "p1-echarts-line.bmp"));
        Render(echarts, zrender, DemoCatalog.Echarts.First(item => item.Id == "bar"), 480, 360, Path.Combine(artifacts, "p1-echarts-bar.bmp"));
        Render(echarts, zrender, DemoCatalog.Echarts.First(item => item.Id == "pie"), 480, 360, Path.Combine(artifacts, "p1-echarts-pie.bmp"));
        Render(echarts, zrender, DemoCatalog.Echarts.First(item => item.Id == "scatter"), 480, 360, Path.Combine(artifacts, "p1-echarts-scatter.bmp"));
        Console.WriteLine("P1 smoke ok");
        return 0;
    }

    private static void Render(
        WasmEchartsModule echarts,
        WasmZrenderModule zrender,
        DemoEntry entry,
        int width,
        int height,
        string path)
    {
        var demo = DemoRegistry.Resolve(entry);
        IDisposable? tracked = null;
        var result = demo.Render(new RenderContext
        {
            Echarts = echarts,
            Zrender = zrender,
            Width = width,
            Height = height,
            Track = item => tracked = item,
        });
        using (tracked)
        {
            if (result.Error is not null)
            {
                throw new InvalidOperationException($"{entry.Id}: {result.Error}");
            }

            if (result.Rgba is null || result.Rgba.Length != result.Width * result.Height * 4)
            {
                throw new InvalidOperationException($"{entry.Id}: bad RGBA size {result.Rgba?.Length}");
            }

            WriteBmp(path, result.Width, result.Height, result.Rgba);
            Console.WriteLine($"{entry.Id} -> {path}");
        }
    }

    private static string FindDemoRoot()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir is not null)
        {
            if (File.Exists(Path.Combine(dir.FullName, "NetCoreDemo.slnx")))
            {
                return dir.FullName;
            }

            dir = dir.Parent;
        }

        return Path.GetFullPath(Path.Combine(AppContext.BaseDirectory, "..", "..", "..", ".."));
    }

    private static void WriteBmp(string path, int width, int height, byte[] rgba)
    {
        var rowStride = width * 4;
        var imageSize = rowStride * height;
        using var stream = File.Create(path);
        Span<byte> header = stackalloc byte[54];
        header.Clear();
        header[0] = (byte)'B';
        header[1] = (byte)'M';
        System.Buffers.Binary.BinaryPrimitives.WriteInt32LittleEndian(header[2..], 54 + imageSize);
        System.Buffers.Binary.BinaryPrimitives.WriteInt32LittleEndian(header[10..], 54);
        System.Buffers.Binary.BinaryPrimitives.WriteInt32LittleEndian(header[14..], 40);
        System.Buffers.Binary.BinaryPrimitives.WriteInt32LittleEndian(header[18..], width);
        System.Buffers.Binary.BinaryPrimitives.WriteInt32LittleEndian(header[22..], height);
        System.Buffers.Binary.BinaryPrimitives.WriteInt16LittleEndian(header[26..], 1);
        System.Buffers.Binary.BinaryPrimitives.WriteInt16LittleEndian(header[28..], 32);
        System.Buffers.Binary.BinaryPrimitives.WriteInt32LittleEndian(header[34..], imageSize);
        stream.Write(header);
        var row = new byte[rowStride];
        for (var y = height - 1; y >= 0; y--)
        {
            var src = y * rowStride;
            for (var x = 0; x < width; x++)
            {
                var i = src + x * 4;
                var o = x * 4;
                row[o] = rgba[i + 2];
                row[o + 1] = rgba[i + 1];
                row[o + 2] = rgba[i];
                row[o + 3] = rgba[i + 3];
            }

            stream.Write(row);
        }
    }
}

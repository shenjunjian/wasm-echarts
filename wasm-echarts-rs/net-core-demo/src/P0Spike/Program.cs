using WasmCharts.Runtime;

internal static class Program
{
    private static int Main()
    {
        var artifacts = Path.GetFullPath(Path.Combine(FindDemoRoot(), "artifacts"));
        Directory.CreateDirectory(artifacts);

        var fontPath = AssetPaths.FindFont("simkai.ttf");
        var font = File.ReadAllBytes(fontPath);
        Console.WriteLine($"font: {fontPath} ({font.Length} bytes)");

        var echartsBmp = Path.Combine(artifacts, "p0-echarts-line.bmp");
        RenderEcharts(echartsBmp, font);

        var zrenderBmp = Path.Combine(artifacts, "p0-zrender-rect.bmp");
        RenderZrender(zrenderBmp);

        Console.WriteLine("P0 spike ok");
        Console.WriteLine(echartsBmp);
        Console.WriteLine(zrenderBmp);
        return 0;
    }

    private static void RenderEcharts(string outPath, byte[] font)
    {
        using var echarts = WasmEchartsModule.Load();
        echarts.RegisterFont(font, new FontOptions
        {
            FamilyName = "Noto Sans SC",
            SansSerif = ["Noto Sans SC"],
        });

        using var chart = echarts.Create(320, 200, dpr: 1);
        chart.SetOption(JsVal.From(new
        {
            title = new { text = "P0 spike" },
            xAxis = new { type = "category", data = new[] { "Mon", "Tue", "Wed" } },
            yAxis = new { type = "value" },
            series = new[]
            {
                new { type = "line", data = new[] { 150, 230, 224 } },
            },
        }));

        var rgba = chart.Refresh();
        const int width = 320;
        const int height = 200;
        if (rgba.Length != width * height * 4)
        {
            throw new InvalidOperationException($"echarts RGBA size {rgba.Length}, expected {width * height * 4}");
        }

        var painted = RgbaBmp.CountNonTransparent(rgba);
        if (painted == 0)
        {
            throw new InvalidOperationException("echarts refresh produced an empty image");
        }

        RgbaBmp.Write(outPath, width, height, rgba);
        Console.WriteLine($"echarts: {painted} opaque pixels -> {outPath}");
    }

    private static void RenderZrender(string outPath)
    {
        using var zrender = WasmZrenderModule.Load();
        using var zr = zrender.Init(new { width = 320, height = 200, devicePixelRatio = 1 });
        var rect = zrender.Rect(new
        {
            shape = new { x = 40, y = 30, width = 240, height = 140 },
            style = new { fill = "#d94c4c" },
        });
        zr.Add(rect);

        var rgba = zr.Refresh();
        const int width = 320;
        const int height = 200;
        if (rgba.Length != width * height * 4)
        {
            throw new InvalidOperationException($"zrender RGBA size {rgba.Length}, expected {width * height * 4}");
        }

        var painted = RgbaBmp.CountNonTransparent(rgba);
        if (painted == 0)
        {
            throw new InvalidOperationException("zrender refresh produced an empty image");
        }

        RgbaBmp.Write(outPath, width, height, rgba);
        Console.WriteLine($"zrender: {painted} opaque pixels -> {outPath}");
    }

    private static string FindDemoRoot()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir is not null)
        {
            if (File.Exists(Path.Combine(dir.FullName, "NetCoreDemo.slnx"))
                || File.Exists(Path.Combine(dir.FullName, "NetCoreDemo.sln")))
            {
                return dir.FullName;
            }

            dir = dir.Parent;
        }

        return Path.GetFullPath(Path.Combine(AppContext.BaseDirectory, "..", "..", "..", ".."));
    }
}

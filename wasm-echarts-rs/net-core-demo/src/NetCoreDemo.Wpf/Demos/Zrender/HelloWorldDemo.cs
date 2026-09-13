using WasmCharts.Runtime;

namespace NetCoreDemo.Wpf.Demos.Zrender;

internal sealed class HelloWorldDemo : IDemo
{
    public string Source => """
        using var zr = zrender.Init(new { width, height, devicePixelRatio = 1 });
        var w = zr.Width;
        var h = zr.Height;

        var sun = zrender.Circle(new
        {
            shape = new { cx = 0, cy = 0, r = 50 },
            style = new { fill = "#FF904F" },
            position = new[] { w / 2.0, h / 2.0 },
        });
        zr.Add(sun);

        var water = zrender.Rect(new
        {
            shape = new { x = 0, y = 0, width = w, height = h / 2.0 },
            style = new
            {
                fill = zrender.RadialGradient(0.5, -0.1, 1, new object[]
                {
                    new { offset = 0, color = "#FFB166" },
                    new { offset = 0.2, color = "#D7C467" },
                    new { offset = 1, color = "#37B0FF" },
                }),
            },
            position = new[] { 0.0, h / 2.0 },
        });
        zr.Add(water);

        var sky = zrender.Rect(new
        {
            shape = new { x = 0, y = 0, width = w, height = h },
            style = new { fill = "#D7F9FF" },
            zlevel = -1,
        });
        zr.Add(sky);

        byte[] rgba = zr.Refresh();
        """;

    public RenderResult Render(RenderContext context)
    {
        var zr = context.Zrender.Init(new
        {
            width = context.Width,
            height = context.Height,
            devicePixelRatio = 1,
        });
        context.Track(zr);

        var w = zr.Width;
        var h = zr.Height;

        zr.Add(context.Zrender.Circle(new
        {
            shape = new { cx = 0, cy = 0, r = 50 },
            style = new { fill = "#FF904F" },
            position = new[] { w / 2.0, h / 2.0 },
        }));

        zr.Add(context.Zrender.Rect(new
        {
            shape = new { x = 0, y = 0, width = w, height = h / 2.0 },
            style = new
            {
                fill = context.Zrender.RadialGradient(0.5, -0.1, 1, new object[]
                {
                    new { offset = 0, color = "#FFB166" },
                    new { offset = 0.2, color = "#D7C467" },
                    new { offset = 1, color = "#37B0FF" },
                }),
            },
            position = new[] { 0.0, h / 2.0 },
        }));

        zr.Add(context.Zrender.Rect(new
        {
            shape = new { x = 0, y = 0, width = w, height = h },
            style = new { fill = "#D7F9FF" },
            zlevel = -1,
        }));

        var rgba = zr.Refresh();
        return new RenderResult(rgba, (int)w, (int)h, null);
    }
}

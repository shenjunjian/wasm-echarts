namespace NetCoreDemo.Wpf.Demos.Echarts;

internal sealed class BarDemo : IDemo
{
    public string Source => """
        echarts.RegisterDefaultFont();
        using var chart = echarts.Create(480, 360);
        chart.SetOption(new
        {
            xAxis = new
            {
                type = "category",
                data = new[] { "A", "B", "C", "D" },
                axisLabel = new { formatter = "{value}" },
            },
            yAxis = new { type = "value", axisLabel = new { formatter = "{value}" } },
            series = new[]
            {
                new
                {
                    type = "bar",
                    name = "数量",
                    data = new[] { 40, 90, 60, 120 },
                    label = new { show = true },
                },
            },
        });
        byte[] rgba = chart.Refresh();
        """;

    public RenderResult Render(RenderContext context)
    {
        context.Echarts.RegisterDefaultFont();
        var chart = context.Echarts.Create((uint)context.Width, (uint)context.Height);
        context.Track(chart);
        chart.SetOption(new
        {
            xAxis = new
            {
                type = "category",
                data = new[] { "A", "B", "C", "D" },
                axisLabel = new { formatter = "{value}" },
            },
            yAxis = new { type = "value", axisLabel = new { formatter = "{value}" } },
            series = new[]
            {
                new
                {
                    type = "bar",
                    name = "数量",
                    data = new[] { 40, 90, 60, 120 },
                    label = new { show = true },
                },
            },
        });
        return new RenderResult(chart.Refresh(), context.Width, context.Height, null);
    }
}

namespace NetCoreDemo.Wpf.Demos.Echarts;

internal sealed class LineDemo : IDemo
{
    public string Source => """
        echarts.RegisterDefaultFont();
        using var chart = echarts.Create(480, 360);
        chart.SetOption(new
        {
            xAxis = new
            {
                type = "category",
                data = new[] { "Mon", "Tue", "Wed", "Thu", "Fri" },
                axisLabel = new { formatter = "{value}" },
            },
            yAxis = new { type = "value" },
            series = new[]
            {
                new
                {
                    type = "line",
                    name = "销量",
                    data = new[] { 120, 200, 150, 80, 70 },
                    symbol = "emptyCircle",
                    symbolSize = 8,
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
                data = new[] { "Mon", "Tue", "Wed", "Thu", "Fri" },
                axisLabel = new { formatter = "{value}" },
            },
            yAxis = new { type = "value" },
            series = new[]
            {
                new
                {
                    type = "line",
                    name = "销量",
                    data = new[] { 120, 200, 150, 80, 70 },
                    symbol = "emptyCircle",
                    symbolSize = 8,
                    label = new { show = true },
                },
            },
        });
        return new RenderResult(chart.Refresh(), context.Width, context.Height, null);
    }
}

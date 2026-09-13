namespace NetCoreDemo.Wpf.Demos.Echarts;

internal sealed class ScatterDemo : IDemo
{
    public string Source => """
        echarts.RegisterDefaultFont();
        using var chart = echarts.Create(480, 360);
        chart.SetOption(new
        {
            xAxis = new { type = "value", scale = true },
            yAxis = new { type = "value", scale = true },
            series = new[]
            {
                new
                {
                    type = "scatter",
                    name = "样本",
                    symbol = "circle",
                    symbolSize = 12,
                    data = new[]
                    {
                        new[] { 10.0, 8.04 },
                        new[] { 8.07, 6.95 },
                        new[] { 13.0, 7.58 },
                        new[] { 9.05, 8.81 },
                        new[] { 11.0, 8.33 },
                        new[] { 14.0, 7.66 },
                        new[] { 13.4, 6.81 },
                        new[] { 10.0, 6.33 },
                        new[] { 14.0, 8.96 },
                        new[] { 12.5, 6.82 },
                    },
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
            xAxis = new { type = "value", scale = true },
            yAxis = new { type = "value", scale = true },
            series = new[]
            {
                new
                {
                    type = "scatter",
                    name = "样本",
                    symbol = "circle",
                    symbolSize = 12,
                    data = new[]
                    {
                        new[] { 10.0, 8.04 },
                        new[] { 8.07, 6.95 },
                        new[] { 13.0, 7.58 },
                        new[] { 9.05, 8.81 },
                        new[] { 11.0, 8.33 },
                        new[] { 14.0, 7.66 },
                        new[] { 13.4, 6.81 },
                        new[] { 10.0, 6.33 },
                        new[] { 14.0, 8.96 },
                        new[] { 12.5, 6.82 },
                    },
                },
            },
        });
        return new RenderResult(chart.Refresh(), context.Width, context.Height, null);
    }
}

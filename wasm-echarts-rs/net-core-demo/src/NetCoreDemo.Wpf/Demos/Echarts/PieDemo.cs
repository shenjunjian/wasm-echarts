namespace NetCoreDemo.Wpf.Demos.Echarts;

internal sealed class PieDemo : IDemo
{
    public string Source => """
        echarts.RegisterDefaultFont();
        using var chart = echarts.Create(480, 360);
        chart.SetOption(new
        {
            series = new[]
            {
                new
                {
                    type = "pie",
                    name = "占比",
                    radius = new[] { "30%", "55%" },
                    center = new[] { "50%", "50%" },
                    label = new { show = true, formatter = "{b}: {d}%" },
                    data = new object[]
                    {
                        new { name = "直接访问", value = 335 },
                        new { name = "邮件营销", value = 310 },
                        new { name = "联盟广告", value = 234 },
                        new { name = "视频广告", value = 135 },
                        new { name = "搜索引擎", value = 1548 },
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
            series = new[]
            {
                new
                {
                    type = "pie",
                    name = "占比",
                    radius = new[] { "30%", "55%" },
                    center = new[] { "50%", "50%" },
                    label = new { show = true, formatter = "{b}: {d}%" },
                    data = new object[]
                    {
                        new { name = "直接访问", value = 335 },
                        new { name = "邮件营销", value = 310 },
                        new { name = "联盟广告", value = 234 },
                        new { name = "视频广告", value = 135 },
                        new { name = "搜索引擎", value = 1548 },
                    },
                },
            },
        });
        return new RenderResult(chart.Refresh(), context.Width, context.Height, null);
    }
}

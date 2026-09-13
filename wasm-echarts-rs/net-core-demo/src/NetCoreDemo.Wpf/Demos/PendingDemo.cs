using NetCoreDemo.Wpf.Catalog;

namespace NetCoreDemo.Wpf.Demos;

internal sealed class PendingDemo : IDemo
{
    private readonly DemoEntry _entry;

    public PendingDemo(DemoEntry entry) => _entry = entry;

    public string Source =>
        $"""
        // {_entry.Id}
        // {_entry.Title}
        //
        // 状态: {DemoCatalog.StatusLabel(_entry.Status)}
        // 站点文件: site/{(_entry.Product == DemoProduct.Zrender ? "zrender" : "echarts")}/examples/{_entry.Id}.js
        //
        // {Message()}
        """;

    public RenderResult Render(RenderContext context)
    {
        return new RenderResult(null, context.Width, context.Height, Message());
    }

    private string Message()
    {
        return _entry.Status switch
        {
            DemoStatus.UnsupportedHost =>
                "宿主不支持：该示例依赖浏览器 DOM / 百度地图等，NetCore Wasmtime 路径无法运行。菜单保留以便与站点目录对齐。",
            DemoStatus.NeedsCallback =>
                "需要 JS 回调（formatter / renderItem 等）。P3 会用 C# JsFunction 覆盖能写的部分；写不了的仍保留菜单项。",
            DemoStatus.OptionOnly =>
                "仅有 JSON option，尚未接到 Runtime SetOption。",
            _ => _entry.Product == DemoProduct.Zrender
                ? "P2 将按站点逻辑移植该 wasm-zrender 示例。"
                : "P3 将按站点分组接入该 wasm-echarts 示例。",
        };
    }
}

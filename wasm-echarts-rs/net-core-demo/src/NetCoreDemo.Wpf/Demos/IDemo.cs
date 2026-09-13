using WasmCharts.Runtime;

namespace NetCoreDemo.Wpf.Demos;

internal sealed class RenderContext
{
    public required WasmEchartsModule Echarts { get; init; }
    public required WasmZrenderModule Zrender { get; init; }
    public required int Width { get; init; }
    public required int Height { get; init; }
    public required Action<IDisposable> Track { get; init; }
}

internal sealed record RenderResult(byte[]? Rgba, int Width, int Height, string? Error);

internal interface IDemo
{
    RenderResult Render(RenderContext context);
    string Source { get; }
}

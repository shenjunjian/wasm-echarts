using NetCoreDemo.Wpf.Catalog;
using NetCoreDemo.Wpf.Demos.Echarts;
using NetCoreDemo.Wpf.Demos.Zrender;

namespace NetCoreDemo.Wpf.Demos;

internal static class DemoRegistry
{
    private static readonly Dictionary<string, IDemo> Ready = new(StringComparer.Ordinal)
    {
        ["hello_world"] = new HelloWorldDemo(),
        ["line"] = new LineDemo(),
        ["bar"] = new BarDemo(),
        ["pie"] = new PieDemo(),
        ["scatter"] = new ScatterDemo(),
    };

    public static IDemo Resolve(DemoEntry entry)
    {
        return Ready.TryGetValue(entry.Id, out var demo) ? demo : new PendingDemo(entry);
    }
}

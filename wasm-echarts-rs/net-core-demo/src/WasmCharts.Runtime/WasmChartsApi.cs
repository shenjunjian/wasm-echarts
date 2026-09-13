using Wasmtime;

namespace WasmCharts.Runtime;

public static class ChartAssets
{
    public const string DefaultFamily = "Noto Sans SC";

    public static string WasmEchartsPath => AssetPaths.FindWasmEcharts();

    public static string WasmZrenderPath => AssetPaths.FindWasmZrender();

    public static string DefaultFontPath => AssetPaths.FindFont("NotoSansSC-Regular.ttf");

    public static string FindFont(string fileName) => AssetPaths.FindFont(fileName);

    public static FontOptions DefaultFontOptions { get; } = new()
    {
        FamilyName = DefaultFamily,
        SansSerif = [DefaultFamily],
    };
}

public sealed class FontOptions
{
    public string? FamilyName { get; init; }
    public IReadOnlyList<string>? SansSerif { get; init; }

    internal JsVal ToJs()
    {
        var obj = JsVal.NewObject();
        if (FamilyName is not null)
        {
            obj.Set("familyName", JsVal.FromString(FamilyName));
        }

        if (SansSerif is { Count: > 0 })
        {
            var arr = JsVal.NewArray();
            foreach (var name in SansSerif)
            {
                arr.Push(JsVal.FromString(name));
            }

            obj.Set("sansSerif", arr);
        }

        return obj;
    }
}

public sealed class ZElement
{
    internal ZElement(JsVal value, int ptr, uint id)
    {
        Value = value;
        Ptr = ptr;
        Id = id;
    }

    public JsVal Value { get; }
    public int Ptr { get; }
    public uint Id { get; }

    public static implicit operator JsVal(ZElement element) => element.Value;
}

public sealed class WasmEchartsModule : IDisposable
{
    private readonly WasmSession _session;
    private bool _disposed;
    private bool _defaultFont;

    private WasmEchartsModule(WasmSession session) => _session = session;

    public static WasmEchartsModule Load(string? wasmPath = null)
    {
        return new WasmEchartsModule(WasmSession.Load(wasmPath ?? ChartAssets.WasmEchartsPath));
    }

    public void RegisterFont(byte[] data, FontOptions? options = null)
    {
        _session.RegisterFont(data, options?.ToJs() ?? JsVal.NewObject());
    }

    public void RegisterDefaultFont()
    {
        if (_defaultFont)
        {
            return;
        }

        RegisterFont(File.ReadAllBytes(ChartAssets.DefaultFontPath), ChartAssets.DefaultFontOptions);
        _defaultFont = true;
    }

    public EChartsChart Create(uint width, uint height, double dpr = 1)
    {
        var ptr = _session.NewClass("echartsinstance_new", (int)width, (int)height, dpr);
        return new EChartsChart(_session, ptr);
    }

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }

        _disposed = true;
        _session.Dispose();
    }
}

public sealed class EChartsChart : IDisposable
{
    private readonly WasmSession _session;
    private readonly int _ptr;
    private bool _disposed;

    internal EChartsChart(WasmSession session, int ptr)
    {
        _session = session;
        _ptr = ptr;
    }

    public void SetOption(object option, object? opts = null)
    {
        SetOption(JsVal.From(option), opts is null ? null : JsVal.From(opts));
    }

    public void SetOption(JsVal option, JsVal? opts = null)
    {
        EnsureAlive();
        var optsIndex = 0;
        if (opts is not null && !opts.IsUndefined && !opts.IsNull)
        {
            optsIndex = Convert.ToInt32(_session.Require("__externref_table_alloc").Invoke());
            _session.Externrefs.SetElement((uint)optsIndex, opts);
        }

        _session.CallFallible("echartsinstance_set_option", _ptr, ValueBox.AsBox(option), optsIndex);
    }

    public byte[] Refresh()
    {
        EnsureAlive();
        return _session.CallRefresh("echartsinstance_refresh", _ptr);
    }

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }

        _disposed = true;
        _session.Dispose(_ptr, "echartsinstance_dispose");
    }

    private void EnsureAlive()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
    }
}

public sealed class WasmZrenderModule : IDisposable
{
    private readonly WasmSession _session;
    private bool _disposed;
    private bool _defaultFont;

    private WasmZrenderModule(WasmSession session) => _session = session;

    public static WasmZrenderModule Load(string? wasmPath = null)
    {
        return new WasmZrenderModule(WasmSession.Load(wasmPath ?? ChartAssets.WasmZrenderPath));
    }

    public void RegisterFont(byte[] data, FontOptions? options = null)
    {
        _session.RegisterFont(data, options?.ToJs() ?? JsVal.NewObject());
    }

    public void RegisterDefaultFont()
    {
        if (_defaultFont)
        {
            return;
        }

        RegisterFont(File.ReadAllBytes(ChartAssets.DefaultFontPath), ChartAssets.DefaultFontOptions);
        _defaultFont = true;
    }

    public ZRenderSurface Init(object? opts = null)
    {
        var js = opts as JsVal ?? JsVal.From(opts ?? JsVal.NewObject());
        var ptr = _session.NewClass("init", ValueBox.AsBox<object>(null!), ValueBox.AsBox(js));
        return new ZRenderSurface(_session, ptr);
    }

    public ZElement Circle(object opts) => CreateShape("circle", opts);

    public ZElement Rect(object opts) => CreateShape("rect", opts);

    public ZElement Sector(object opts) => CreateShape("sector", opts);

    public ZElement Heart(object opts) => CreateShape("heart", opts);

    public ZElement Text(object opts) => CreateShape("text", opts);

    public ZElement Group(object? opts = null) => CreateShape("group", opts ?? JsVal.NewObject(), constructorArgs: false);

    public JsVal RadialGradient(double x, double y, double r, object colorStops, bool global = false, double r0 = 0)
    {
        return JsVal.Obj(
            ("type", "radial"),
            ("x", x),
            ("y", y),
            ("r", r),
            ("r0", r0),
            ("global", global),
            ("colorStops", colorStops));
    }

    public JsVal LinearGradient(double x, double y, double x2, double y2, object colorStops, bool global = false)
    {
        return JsVal.Obj(
            ("type", "linear"),
            ("x", x),
            ("y", y),
            ("x2", x2),
            ("y2", y2),
            ("global", global),
            ("colorStops", colorStops));
    }

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }

        _disposed = true;
        _session.Dispose();
    }

    private ZElement CreateShape(string prefix, object opts, bool constructorArgs = true)
    {
        var js = opts as JsVal ?? JsVal.From(opts);
        var ptr = constructorArgs
            ? _session.NewClass($"{prefix}_new", ValueBox.AsBox(js))
            : _session.NewClass($"{prefix}_new");
        var id = (uint)_session.CallI32($"{prefix}_id", ptr);
        var handle = JsVal.WrapHandle(prefix, ptr);
        handle.Set("id", JsVal.FromNumber(id));
        handle.Set("type", JsVal.FromString(prefix));
        return new ZElement(handle, ptr, id);
    }
}

public sealed class ZRenderSurface : IDisposable
{
    private readonly WasmSession _session;
    private readonly int _ptr;
    private bool _disposed;

    internal ZRenderSurface(WasmSession session, int ptr)
    {
        _session = session;
        _ptr = ptr;
    }

    public uint Width
    {
        get
        {
            EnsureAlive();
            return (uint)_session.CallI32("zrender_getWidth", _ptr);
        }
    }

    public uint Height
    {
        get
        {
            EnsureAlive();
            return (uint)_session.CallI32("zrender_getHeight", _ptr);
        }
    }

    public void Add(ZElement element) => Add(element.Value);

    public void Add(JsVal element)
    {
        EnsureAlive();
        _session.CallFallible("zrender_add", _ptr, ValueBox.AsBox(element));
    }

    public byte[] Refresh()
    {
        EnsureAlive();
        return _session.CallRefresh("zrender_refresh", _ptr);
    }

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }

        _disposed = true;
        _session.Dispose(_ptr, "zrender_dispose");
    }

    private void EnsureAlive()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
    }
}

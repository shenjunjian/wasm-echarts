using Wasmtime;

namespace WasmCharts.Runtime;

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

public sealed class WasmEchartsModule : IDisposable
{
    private readonly WasmSession _session;
    private bool _disposed;

    private WasmEchartsModule(WasmSession session) => _session = session;

    public static WasmEchartsModule Load(string? wasmPath = null)
    {
        return new WasmEchartsModule(WasmSession.Load(wasmPath ?? AssetPaths.FindWasmEcharts()));
    }

    public void RegisterFont(byte[] data, FontOptions? options = null)
    {
        _session.RegisterFont(data, options?.ToJs() ?? JsVal.NewObject());
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

    private WasmZrenderModule(WasmSession session) => _session = session;

    public static WasmZrenderModule Load(string? wasmPath = null)
    {
        return new WasmZrenderModule(WasmSession.Load(wasmPath ?? AssetPaths.FindWasmZrender()));
    }

    public void RegisterFont(byte[] data, FontOptions? options = null)
    {
        _session.RegisterFont(data, options?.ToJs() ?? JsVal.NewObject());
    }

    public ZRenderSurface Init(JsVal? opts = null)
    {
        var ptr = _session.NewClass("init", ValueBox.AsBox<object>(null!), ValueBox.AsBox(opts ?? JsVal.NewObject()));
        return new ZRenderSurface(_session, ptr);
    }

    public int RectNew(JsVal opts)
    {
        return _session.NewClass("rect_new", ValueBox.AsBox(opts));
    }

    public int RectId(int ptr) => _session.CallI32("rect_id", ptr);

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

    public void Add(JsVal element)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        _session.CallFallible("zrender_add", _ptr, ValueBox.AsBox(element));
    }

    public byte[] Refresh()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
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
}

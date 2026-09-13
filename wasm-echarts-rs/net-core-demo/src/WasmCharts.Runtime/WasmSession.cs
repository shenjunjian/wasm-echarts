using System.Buffers.Binary;
using System.Runtime.CompilerServices;
using Wasmtime;

[assembly: InternalsVisibleTo("P0Spike")]

namespace WasmCharts.Runtime;

internal sealed class WasmSession : IDisposable
{
    private readonly Engine _engine;
    private readonly Linker _linker;
    private readonly Module _module;
    private bool _disposed;

    public Store Store { get; }
    public Instance Instance { get; }
    public Memory Memory { get; }
    public Table Externrefs { get; }
    public WbgHost Host { get; }

    private WasmSession(Engine engine, Linker linker, Module module, Store store, Instance instance, WbgHost host)
    {
        _engine = engine;
        _linker = linker;
        _module = module;
        Store = store;
        Instance = instance;
        Host = host;
        Memory = instance.GetMemory("memory")
            ?? throw new WasmHostException("WASM module does not export memory");
        Externrefs = instance.GetTable("__wbindgen_externrefs")
            ?? throw new WasmHostException("WASM module does not export __wbindgen_externrefs");
    }

    public static WasmSession Load(string wasmPath)
    {
        if (!File.Exists(wasmPath))
        {
            throw new FileNotFoundException("WASM module not found", wasmPath);
        }

        var engine = new Engine(new Config()
            .WithReferenceTypes(true)
            .WithMultiValue(true)
            .WithBulkMemory(true)
            .WithSIMD(true));
        var module = Module.FromFile(engine, wasmPath);
        var store = new Store(engine);
        var host = new WbgHost();
        store.SetData(host);
        var linker = new Linker(engine);
        WbgImports.Define(linker, module, host);
        var instance = linker.Instantiate(store, module);
        host.Attach(instance);
        return new WasmSession(engine, linker, module, store, instance, host);
    }

    public int Malloc(int size, int align = 1)
    {
        var malloc = Require("__wbindgen_malloc");
        return Convert.ToInt32(malloc.Invoke(size, align));
    }

    public void Free(int ptr, int len, int align = 1)
    {
        Require("__wbindgen_free").Invoke(ptr, len, align);
    }

    public void WriteBytes(int ptr, ReadOnlySpan<byte> data)
    {
        data.CopyTo(Memory.GetSpan(ptr, data.Length));
    }

    public byte[] ReadBytes(int ptr, int len)
    {
        if (len <= 0)
        {
            return [];
        }

        return Memory.GetSpan(ptr, len).ToArray();
    }

    public object? TakeExternref(int index)
    {
        var value = Externrefs.GetElement((uint)index);
        Require("__externref_table_dealloc").Invoke(index);
        return value;
    }

    public string ErrorMessage(object? value)
    {
        var js = JsVal.Coerce(value);
        if (js.IsStringKind)
        {
            return js.String ?? "";
        }

        if (!string.IsNullOrEmpty(js.ErrorMessage))
        {
            return js.ErrorMessage;
        }

        return js.ToString();
    }

    public int CallI32(string name, params ValueBox[] args)
    {
        return Convert.ToInt32(Require(name).Invoke(args));
    }

    public ValueBox[] CallMany(string name, params ValueBox[] args)
    {
        var ret = Require(name).Invoke(args);
        return NormalizeMany(ret);
    }

    public int NewClass(string export, params ValueBox[] args)
    {
        var ret = CallMany(export, args);
        ThrowIfError(ret, ptrIndex: 0, errIndex: 1, flagIndex: 2);
        return ToI32(ret[0]);
    }

    public void CallFallible(string export, params ValueBox[] args)
    {
        var ret = CallMany(export, args);
        ThrowIfError(ret, ptrIndex: 0, errIndex: 0, flagIndex: 1);
    }

    public byte[] CallRefresh(string export, int ptr)
    {
        var ret = CallMany(export, ptr);
        ThrowIfError(ret, ptrIndex: 0, errIndex: 2, flagIndex: 3);
        var dataPtr = ToI32(ret[0]);
        var len = ToI32(ret[1]);
        var bytes = ReadBytes(dataPtr, len);
        Free(dataPtr, len, 1);
        return bytes;
    }

    public void RegisterFont(byte[] data, JsVal opts)
    {
        var ptr = Malloc(data.Length);
        WriteBytes(ptr, data);
        CallFallible("registerFont", ptr, data.Length, ValueBox.AsBox(opts));
    }

    public void Dispose(int ptr, string export)
    {
        Require(export).Invoke(ptr);
    }

    public Function Require(string name)
    {
        return Instance.GetFunction(name)
            ?? throw new WasmHostException($"missing WASM export '{name}'");
    }

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }

        _disposed = true;
        Store.Dispose();
        _linker.Dispose();
        _module.Dispose();
        _engine.Dispose();
    }

    private void ThrowIfError(ValueBox[] ret, int ptrIndex, int errIndex, int flagIndex)
    {
        if (flagIndex >= ret.Length)
        {
            return;
        }

        if (ToI32(ret[flagIndex]) == 0)
        {
            return;
        }

        var errIdx = ToI32(ret[errIndex]);
        throw new WasmHostException(ErrorMessage(TakeExternref(errIdx)));
    }

    private static ValueBox[] NormalizeMany(object? ret)
    {
        switch (ret)
        {
            case null:
                return [];
            case ValueBox box:
                return [box];
            case ValueBox[] boxes:
                return boxes;
            case object[] arr:
                return arr.Select(ToValueBox).ToArray();
            default:
                return [ToValueBox(ret)];
        }
    }

    private static ValueBox ToValueBox(object? value) => value switch
    {
        null => ValueBox.AsBox<object>(null!),
        ValueBox box => box,
        int i => i,
        uint u => (int)u,
        long l => l,
        float f => f,
        double d => d,
        string s => s,
        _ => ValueBox.AsBox(value),
    };

    private static int ToI32(ValueBox box)
    {
        try
        {
            return box.AsInt32();
        }
        catch
        {
            return Convert.ToInt32(box.As<object>());
        }
    }
}

internal static class AssetPaths
{
    public static string FindWasmEcharts() => Find("crates", "wasm-echarts", "pkg", "wasm_echarts_bg.wasm");

    public static string FindWasmZrender() => Find("crates", "wasm-zrender", "pkg", "wasm_zrender_bg.wasm");

    public static string FindFont(string fileName) => Find("site", "public", "fonts", fileName);

    private static string Find(params string[] relative)
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir is not null)
        {
            var candidate = Path.Combine(new[] { dir.FullName }.Concat(relative).ToArray());
            if (File.Exists(candidate))
            {
                return candidate;
            }

            var nested = Path.Combine(new[] { dir.FullName, "wasm-echarts-rs" }.Concat(relative).ToArray());
            if (File.Exists(nested))
            {
                return nested;
            }

            dir = dir.Parent;
        }

        throw new FileNotFoundException("asset not found: " + Path.Combine(relative));
    }
}

internal static class RgbaBmp
{
    public static void Write(string path, int width, int height, byte[] rgba)
    {
        if (rgba.Length < width * height * 4)
        {
            throw new ArgumentException("RGBA buffer is smaller than width*height*4", nameof(rgba));
        }

        var rowStride = width * 4;
        var imageSize = rowStride * height;
        var fileSize = 54 + imageSize;
        using var stream = File.Create(path);
        Span<byte> header = stackalloc byte[54];
        header.Clear();
        header[0] = (byte)'B';
        header[1] = (byte)'M';
        BinaryPrimitives.WriteInt32LittleEndian(header[2..], fileSize);
        BinaryPrimitives.WriteInt32LittleEndian(header[10..], 54);
        BinaryPrimitives.WriteInt32LittleEndian(header[14..], 40);
        BinaryPrimitives.WriteInt32LittleEndian(header[18..], width);
        BinaryPrimitives.WriteInt32LittleEndian(header[22..], height);
        BinaryPrimitives.WriteInt16LittleEndian(header[26..], 1);
        BinaryPrimitives.WriteInt16LittleEndian(header[28..], 32);
        BinaryPrimitives.WriteInt32LittleEndian(header[34..], imageSize);
        stream.Write(header);
        var row = new byte[rowStride];
        for (var y = height - 1; y >= 0; y--)
        {
            var src = y * rowStride;
            for (var x = 0; x < width; x++)
            {
                var i = src + x * 4;
                var o = x * 4;
                row[o] = rgba[i + 2];
                row[o + 1] = rgba[i + 1];
                row[o + 2] = rgba[i];
                row[o + 3] = rgba[i + 3];
            }

            stream.Write(row);
        }
    }

    public static int CountNonTransparent(byte[] rgba)
    {
        var n = 0;
        for (var i = 3; i < rgba.Length; i += 4)
        {
            if (rgba[i] != 0)
            {
                n++;
            }
        }

        return n;
    }
}

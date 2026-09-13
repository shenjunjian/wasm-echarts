using System.Text;
using Wasmtime;

namespace WasmCharts.Runtime;

internal sealed class WbgHost
{
    private Table? _externrefs;
    private bool _tableReady;

    public void Attach(Instance instance)
    {
        var table = instance.GetTable("__wbindgen_externrefs")
            ?? throw new WasmHostException("module does not export __wbindgen_externrefs");
        InitExternrefTable(table);
    }

    public void Invoke(
        WbgKind kind,
        string name,
        Caller caller,
        ReadOnlySpan<ValueBox> args,
        Span<ValueBox> results)
    {
        try
        {
            Dispatch(kind, name, caller, args, results);
        }
        catch (WasmHostException)
        {
            throw;
        }
        catch (Exception ex)
        {
            StoreException(caller, ex);
        }
    }

    private void Dispatch(
        WbgKind kind,
        string name,
        Caller caller,
        ReadOnlySpan<ValueBox> args,
        Span<ValueBox> results)
    {
        switch (kind)
        {
            case WbgKind.BooleanGet:
                SetI32(results, BooleanGet(Obj(args, 0)));
                break;
            case WbgKind.CopyToTypedArray:
                CopyToTypedArray(caller, I32(args, 0), I32(args, 1), Obj(args, 2));
                break;
            case WbgKind.DebugString:
                WriteStringPair(caller, I32(args, 0), JsVal.Coerce(Obj(args, 1)).DebugString());
                break;
            case WbgKind.IsFunction:
                SetI32(results, JsVal.Coerce(Obj(args, 0)).IsFunctionKind ? 1 : 0);
                break;
            case WbgKind.IsNull:
                SetI32(results, ReferenceEquals(JsVal.Coerce(Obj(args, 0)), JsVal.Null) ? 1 : 0);
                break;
            case WbgKind.IsObject:
                SetI32(results, IsJsObject(JsVal.Coerce(Obj(args, 0))) ? 1 : 0);
                break;
            case WbgKind.IsString:
                SetI32(results, JsVal.Coerce(Obj(args, 0)).IsStringKind ? 1 : 0);
                break;
            case WbgKind.IsUndefined:
                SetI32(results, IsUndefined(Obj(args, 0)) ? 1 : 0);
                break;
            case WbgKind.JsvalEq:
                SetI32(results, Same(Obj(args, 0), Obj(args, 1)) ? 1 : 0);
                break;
            case WbgKind.NumberGet:
                NumberGet(caller, I32(args, 0), Obj(args, 1));
                break;
            case WbgKind.StringGet:
                StringGet(caller, I32(args, 0), Obj(args, 1));
                break;
            case WbgKind.Throw:
                throw new WasmHostException(ReadUtf8(caller, I32(args, 0), I32(args, 1)));
            case WbgKind.ClosureUnref:
                if (JsVal.Coerce(Obj(args, 0)).ClosureUnref is { } unref)
                {
                    unref();
                }

                break;
            case WbgKind.AddEventListener:
            case WbgKind.RemoveEventListener:
            case WbgKind.DrawImage:
            case WbgKind.PutImageData:
            case WbgKind.ReleasePointerCapture:
            case WbgKind.SetPointerCapture:
            case WbgKind.SetProperty:
                break;
            case WbgKind.Call1:
                SetObj(results, Call(Obj(args, 0)));
                break;
            case WbgKind.Call2:
                SetObj(results, Call(Obj(args, 0), Obj(args, 2)));
                break;
            case WbgKind.Call3:
                SetObj(results, Call(Obj(args, 0), Obj(args, 2), Obj(args, 3)));
                break;
            case WbgKind.ClientX:
            case WbgKind.ClientY:
            case WbgKind.PointerId:
            case WbgKind.NaturalHeight:
            case WbgKind.NaturalWidth:
                SetI32(results, GetIntProp(Obj(args, 0), kind));
                break;
            case WbgKind.CreateElement:
                SetObj(results, JsVal.NewObject());
                break;
            case WbgKind.WrapCustomSeriesApi:
                SetObj(results, JsVal.WrapHandle("CustomSeriesApi", I32(args, 0)));
                break;
            case WbgKind.WrapElement:
                SetObj(results, JsVal.WrapHandle("Element", I32(args, 0)));
                break;
            case WbgKind.ImageDataBytes:
                WriteBytesPair(caller, I32(args, 0), JsVal.Coerce(Obj(args, 1)).Bytes ?? []);
                break;
            case WbgKind.Document:
            case WbgKind.GetContext:
            case WbgKind.QuerySelector:
            case WbgKind.GlobalThis:
            case WbgKind.Global:
            case WbgKind.Self:
            case WbgKind.Window:
                SetI32(results, 0);
                break;
            case WbgKind.ConsoleError2:
                Console.Error.WriteLine($"{Fmt(Obj(args, 0))} {Fmt(Obj(args, 1))}");
                break;
            case WbgKind.ConsoleErrorString:
                Console.Error.WriteLine(ReadUtf8(caller, I32(args, 0), I32(args, 1)));
                Free(caller, I32(args, 0), I32(args, 1), 1);
                break;
            case WbgKind.ConsoleWarn:
                Console.Error.WriteLine("warn: " + Fmt(Obj(args, 0)));
                break;
            case WbgKind.ArrayFrom:
                SetObj(results, ArrayFrom(JsVal.Coerce(Obj(args, 0))));
                break;
            case WbgKind.GetBoundingClientRect:
                SetObj(results, JsVal.Obj(("left", 0), ("top", 0), ("width", 0), ("height", 0)));
                break;
            case WbgKind.GetImageData:
                SetObj(results, new JsVal(JsValKind.ImageData) { Bytes = [] });
                break;
            case WbgKind.ReflectGet:
                SetObj(results, ReflectGet(Obj(args, 0), Obj(args, 1)));
                break;
            case WbgKind.ArrayGet:
                SetObj(results, JsVal.Coerce(Obj(args, 0)).Get((int)(uint)I32(args, 1)));
                break;
            case WbgKind.InstanceofArray:
            case WbgKind.IsArray:
                SetI32(results, JsVal.Coerce(Obj(args, 0)).IsArrayKind ? 1 : 0);
                break;
            case WbgKind.InstanceofCanvas2d:
            case WbgKind.InstanceofElement:
            case WbgKind.InstanceofCanvas:
            case WbgKind.InstanceofImage:
            case WbgKind.InstanceofWindow:
                SetI32(results, 0);
                break;
            case WbgKind.InstanceofFunction:
                SetI32(results, JsVal.Coerce(Obj(args, 0)).IsFunctionKind ? 1 : 0);
                break;
            case WbgKind.InstanceofObject:
                SetI32(results, InstanceofObject(JsVal.Coerce(Obj(args, 0))) ? 1 : 0);
                break;
            case WbgKind.InstanceofUint8Array:
                SetI32(results, JsVal.Coerce(Obj(args, 0)).IsUint8ArrayKind ? 1 : 0);
                break;
            case WbgKind.ObjectKeys:
            case WbgKind.OwnKeys:
                SetObj(results, KeysOf(JsVal.Coerce(Obj(args, 0))));
                break;
            case WbgKind.DomLeft:
                SetF64(results, NumProp(Obj(args, 0), "left"));
                break;
            case WbgKind.DomTop:
                SetF64(results, NumProp(Obj(args, 0), "top"));
                break;
            case WbgKind.Length:
                SetI32(results, JsVal.Coerce(Obj(args, 0)).Length);
                break;
            case WbgKind.NewError:
                SetObj(results, new JsVal(JsValKind.Error) { ErrorMessage = "", ErrorStack = "" });
                break;
            case WbgKind.NewObject:
                SetObj(results, JsVal.NewObject());
                break;
            case WbgKind.NewArray:
                SetObj(results, JsVal.NewArray());
                break;
            case WbgKind.NewUint8ArrayFromSlice:
                SetObj(results, JsVal.FromUint8Array(ReadBytes(caller, I32(args, 0), I32(args, 1))));
                break;
            case WbgKind.NewArrayWithLength:
                SetObj(results, JsVal.NewArray((int)(uint)I32(args, 0)));
                break;
            case WbgKind.NewImageData:
                SetObj(results, new JsVal(JsValKind.ImageData)
                {
                    Bytes = ReadBytes(caller, I32(args, 0), I32(args, 1)),
                });
                break;
            case WbgKind.DateNow:
                SetF64(results, DateTimeOffset.UtcNow.ToUnixTimeMilliseconds());
                break;
            case WbgKind.TypedArraySetFromWasm:
                CopyToTypedArray(caller, I32(args, 0), I32(args, 1), Obj(args, 2));
                break;
            case WbgKind.ArrayPush:
                SetI32(results, JsVal.Coerce(Obj(args, 0)).Push(JsVal.Coerce(Obj(args, 1))));
                break;
            case WbgKind.ReflectSet:
                ReflectSet(Obj(args, 0), Obj(args, 1), Obj(args, 2));
                SetI32(results, 1);
                break;
            case WbgKind.ArraySet:
                JsVal.Coerce(Obj(args, 0)).Set((int)(uint)I32(args, 1), JsVal.Coerce(Obj(args, 2)));
                break;
            case WbgKind.SetHeight:
                JsVal.Coerce(Obj(args, 0)).Set("height", JsVal.FromNumber((uint)I32(args, 1)));
                break;
            case WbgKind.SetWidth:
                JsVal.Coerce(Obj(args, 0)).Set("width", JsVal.FromNumber((uint)I32(args, 1)));
                break;
            case WbgKind.ErrorStack:
                WriteStringPair(caller, I32(args, 0), JsVal.Coerce(Obj(args, 1)).ErrorStack ?? "");
                break;
            case WbgKind.Style:
                SetObj(results, JsVal.NewObject());
                break;
            case WbgKind.WidthI32:
                SetI32(results, (int)NumProp(Obj(args, 0), "width"));
                break;
            case WbgKind.HeightI32:
                SetI32(results, (int)NumProp(Obj(args, 0), "height"));
                break;
            case WbgKind.WidthF64:
                SetF64(results, NumProp(Obj(args, 0), "width"));
                break;
            case WbgKind.HeightF64:
                SetF64(results, NumProp(Obj(args, 0), "height"));
                break;
            case WbgKind.CastF64:
                SetObj(results, JsVal.FromNumber(F64(args, 0)));
                break;
            case WbgKind.CastString:
                SetObj(results, JsVal.FromString(ReadUtf8(caller, I32(args, 0), I32(args, 1))));
                break;
            case WbgKind.CastClosure:
                SetObj(results, MakeClosure(I32(args, 0), I32(args, 1)));
                break;
            case WbgKind.InitExternrefTable:
                break;
            case WbgKind.NotImplemented:
            default:
                throw new WasmHostException($"unimplemented wbg import: {name} ({kind})");
        }
    }

    private void InitExternrefTable(Table table)
    {
        if (_tableReady && ReferenceEquals(_externrefs, table))
        {
            return;
        }

        var offset = (uint)table.Grow(4, null);
        table.SetElement(0, null);
        table.SetElement(offset + 0, null);
        table.SetElement(offset + 1, JsVal.Null);
        table.SetElement(offset + 2, JsVal.True);
        table.SetElement(offset + 3, JsVal.False);
        _externrefs = table;
        _tableReady = true;
    }

    private static int BooleanGet(object? value)
    {
        var js = JsVal.Coerce(value);
        if (!js.IsBoolean)
        {
            return 0xFFFFFF;
        }

        return js.Bool ? 1 : 0;
    }

    private static void NumberGet(Caller caller, int retPtr, object? value)
    {
        var js = JsVal.Coerce(value);
        var mem = Mem(caller);
        if (js.IsNumber)
        {
            mem.WriteInt32(retPtr, 1);
            mem.WriteDouble(retPtr + 8, js.Number);
        }
        else
        {
            mem.WriteInt32(retPtr, 0);
            mem.WriteDouble(retPtr + 8, 0);
        }
    }

    private static void StringGet(Caller caller, int retPtr, object? value)
    {
        var js = JsVal.Coerce(value);
        var mem = Mem(caller);
        if (!js.IsStringKind)
        {
            mem.WriteInt32(retPtr, 0);
            mem.WriteInt32(retPtr + 4, 0);
            return;
        }

        var (ptr, len) = PassUtf8(caller, js.String ?? "");
        mem.WriteInt32(retPtr, ptr);
        mem.WriteInt32(retPtr + 4, len);
    }

    private static object? ReflectGet(object? target, object? key)
    {
        var obj = JsVal.Coerce(target);
        var keyVal = JsVal.Coerce(key);
        if (keyVal.IsNumber)
        {
            return obj.Get((int)keyVal.Number);
        }

        if (keyVal.IsStringKind)
        {
            return obj.Get(keyVal.String ?? "");
        }

        return JsVal.Undefined;
    }

    private static void ReflectSet(object? target, object? key, object? value)
    {
        var obj = JsVal.Coerce(target);
        var keyVal = JsVal.Coerce(key);
        var js = JsVal.Coerce(value);
        if (keyVal.IsNumber)
        {
            obj.Set((int)keyVal.Number, js);
            return;
        }

        if (keyVal.IsStringKind)
        {
            obj.Set(keyVal.String ?? "", js);
        }
    }

    private static JsVal ArrayFrom(JsVal value)
    {
        if (value.IsArrayKind)
        {
            var copy = JsVal.NewArray();
            for (var i = 0; i < value.Length; i++)
            {
                copy.Push(value.Get(i));
            }

            return copy;
        }

        if (value.IsObjectKind)
        {
            var arr = JsVal.NewArray();
            foreach (var key in value.Keys())
            {
                arr.Push(value.Get(key));
            }

            return arr;
        }

        return JsVal.NewArray();
    }

    private static JsVal KeysOf(JsVal value)
    {
        var arr = JsVal.NewArray();
        foreach (var key in value.Keys())
        {
            arr.Push(JsVal.FromString(key));
        }

        return arr;
    }

    private static object? Call(object? fn, params object?[] rest)
    {
        var args = rest.Select(JsVal.Coerce).ToArray();
        return JsVal.Coerce(fn).Call(args);
    }

    private static JsVal MakeClosure(int a, int b)
    {
        var live = true;
        var fn = JsVal.FromFunction(_ => JsVal.Undefined);
        fn.ClosureUnref = () => live = false;
        _ = (a, b, live);
        return fn;
    }

    private static bool IsJsObject(JsVal value)
    {
        return value.Kind is JsValKind.Object or JsValKind.Array or JsValKind.Handle
            or JsValKind.Uint8Array or JsValKind.ImageData or JsValKind.DomRect or JsValKind.Error;
    }

    private static bool InstanceofObject(JsVal value)
    {
        return IsJsObject(value) || value.IsFunctionKind;
    }

    private static bool IsUndefined(object? value) =>
        value is null || ReferenceEquals(value, JsVal.Undefined) || JsVal.Coerce(value).IsUndefined;

    private static bool Same(object? a, object? b)
    {
        var left = JsVal.Coerce(a);
        var right = JsVal.Coerce(b);
        if (ReferenceEquals(left, right))
        {
            return true;
        }

        if (left.Kind != right.Kind)
        {
            return false;
        }

        return left.Kind switch
        {
            JsValKind.Boolean => left.Bool == right.Bool,
            JsValKind.Number => left.Number.Equals(right.Number),
            JsValKind.String => left.String == right.String,
            _ => false,
        };
    }

    private static double NumProp(object? target, string key)
    {
        var value = JsVal.Coerce(target).Get(key);
        return value.IsNumber ? value.Number : 0;
    }

    private static int GetIntProp(object? target, WbgKind kind)
    {
        var key = kind switch
        {
            WbgKind.ClientX => "clientX",
            WbgKind.ClientY => "clientY",
            WbgKind.PointerId => "pointerId",
            WbgKind.NaturalHeight => "naturalHeight",
            WbgKind.NaturalWidth => "naturalWidth",
            _ => "",
        };
        return (int)NumProp(target, key);
    }

    private void StoreException(Caller caller, Exception ex)
    {
        var js = new JsVal(JsValKind.Error)
        {
            ErrorMessage = ex.Message,
            ErrorStack = ex.ToString(),
        };
        var idx = AllocExternref(caller, js);
        caller.GetFunction("__wbindgen_exn_store")?.Invoke(idx);
    }

    private int AllocExternref(Caller caller, object? value)
    {
        var alloc = caller.GetFunction("__externref_table_alloc")
            ?? throw new WasmHostException("missing __externref_table_alloc");
        var idx = Convert.ToInt32(alloc.Invoke());
        if (_externrefs is null)
        {
            throw new WasmHostException("externref table is not ready");
        }

        _externrefs.SetElement((uint)idx, value);
        return idx;
    }

    private static void CopyToTypedArray(Caller caller, int ptr, int len, object? dest)
    {
        var bytes = JsVal.Coerce(dest).Bytes;
        if (bytes is null)
        {
            return;
        }

        var src = ReadBytes(caller, ptr, len);
        var n = Math.Min(bytes.Length, src.Length);
        Buffer.BlockCopy(src, 0, bytes, 0, n);
    }

    private static void WriteStringPair(Caller caller, int retPtr, string value)
    {
        var (ptr, len) = PassUtf8(caller, value);
        var mem = Mem(caller);
        mem.WriteInt32(retPtr, ptr);
        mem.WriteInt32(retPtr + 4, len);
    }

    private static void WriteBytesPair(Caller caller, int retPtr, byte[] value)
    {
        var ptr = Malloc(caller, value.Length, 1);
        value.CopyTo(Mem(caller).GetSpan(ptr, value.Length));
        var mem = Mem(caller);
        mem.WriteInt32(retPtr, ptr);
        mem.WriteInt32(retPtr + 4, value.Length);
    }

    private static (int Ptr, int Len) PassUtf8(Caller caller, string value)
    {
        var bytes = Encoding.UTF8.GetBytes(value);
        var ptr = Malloc(caller, bytes.Length, 1);
        bytes.CopyTo(Mem(caller).GetSpan(ptr, bytes.Length));
        return (ptr, bytes.Length);
    }

    private static int Malloc(Caller caller, int size, int align)
    {
        var malloc = caller.GetFunction("__wbindgen_malloc")
            ?? throw new WasmHostException("missing __wbindgen_malloc");
        return Convert.ToInt32(malloc.Invoke(size, align));
    }

    private static void Free(Caller caller, int ptr, int len, int align)
    {
        caller.GetFunction("__wbindgen_free")?.Invoke(ptr, len, align);
    }

    private static byte[] ReadBytes(Caller caller, int ptr, int len)
    {
        if (len <= 0)
        {
            return [];
        }

        return Mem(caller).GetSpan(ptr, len).ToArray();
    }

    private static string ReadUtf8(Caller caller, int ptr, int len) =>
        Encoding.UTF8.GetString(ReadBytes(caller, ptr, len));

    private static Memory Mem(Caller caller) =>
        caller.GetMemory("memory") ?? throw new WasmHostException("missing exported memory");

    private static object? Obj(ReadOnlySpan<ValueBox> args, int index)
    {
        if ((uint)index >= (uint)args.Length)
        {
            return null;
        }

        try
        {
            return args[index].As<object>();
        }
        catch
        {
            return null;
        }
    }

    private static int I32(ReadOnlySpan<ValueBox> args, int index) => args[index].AsInt32();

    private static double F64(ReadOnlySpan<ValueBox> args, int index) => args[index].AsDouble();

    private static void SetI32(Span<ValueBox> results, int value)
    {
        if (results.Length > 0)
        {
            results[0] = value;
        }
    }

    private static void SetF64(Span<ValueBox> results, double value)
    {
        if (results.Length > 0)
        {
            results[0] = value;
        }
    }

    private static void SetObj(Span<ValueBox> results, object? value)
    {
        if (results.Length == 0)
        {
            return;
        }

        results[0] = value is null ? ValueBox.AsBox<object>(null!) : ValueBox.AsBox(value);
    }

    private static string Fmt(object? value) => JsVal.Coerce(value).ToString();
}

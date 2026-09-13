using System.Collections;
using System.Globalization;
using System.Reflection;
using System.Text;

namespace WasmCharts.Runtime;

/// <summary>
/// Host-side stand-in for wasm-bindgen <c>JsValue</c> / <c>externref</c>.
/// </summary>
public class JsVal
{
    public static JsVal Undefined { get; } = new(JsValKind.Undefined);
    public static JsVal Null { get; } = new(JsValKind.Null);
    public static JsVal True { get; } = new(true);
    public static JsVal False { get; } = new(false);

    internal JsValKind Kind { get; }
    internal bool Bool { get; }
    internal double Number { get; }
    internal string? String { get; }
    internal List<JsVal>? Array { get; private set; }
    internal Dictionary<string, JsVal>? Object { get; private set; }
    internal List<string>? ObjectKeys { get; private set; }
    internal byte[]? Bytes { get; set; }
    internal Func<JsVal[], JsVal>? Function { get; set; }
    internal string? HandleClass { get; private set; }
    internal int HandlePtr { get; private set; }
    internal string? ErrorMessage { get; set; }
    internal string? ErrorStack { get; set; }
    internal Action? ClosureUnref { get; set; }

    internal JsVal(JsValKind kind) => Kind = kind;

    private JsVal(bool value)
    {
        Kind = JsValKind.Boolean;
        Bool = value;
    }

    internal JsVal(double value)
    {
        Kind = JsValKind.Number;
        Number = value;
    }

    internal JsVal(string value)
    {
        Kind = JsValKind.String;
        String = value;
    }

    public static JsVal FromBoolean(bool value) => value ? True : False;

    public static JsVal FromNumber(double value) => new(value);

    public static JsVal FromString(string value) => new(value);

    public static JsVal FromUint8Array(byte[] bytes) => new(JsValKind.Uint8Array) { Bytes = bytes };

    public static JsVal FromFunction(Func<JsVal[], JsVal> fn) => new(JsValKind.Function) { Function = fn };

    public static JsVal Obj(params (string Key, object? Value)[] entries)
    {
        var obj = NewObject();
        foreach (var (key, value) in entries)
        {
            obj.Set(key, From(value));
        }

        return obj;
    }

    public static JsVal Arr(params object?[] items)
    {
        var arr = NewArray();
        foreach (var item in items)
        {
            arr.Push(From(item));
        }

        return arr;
    }

    public static JsVal NewObject()
    {
        return new JsVal(JsValKind.Object)
        {
            Object = new Dictionary<string, JsVal>(StringComparer.Ordinal),
            ObjectKeys = [],
        };
    }

    public static JsVal NewArray(int length = 0)
    {
        var list = new List<JsVal>(Math.Max(length, 0));
        for (var i = 0; i < length; i++)
        {
            list.Add(Undefined);
        }

        return new JsVal(JsValKind.Array) { Array = list };
    }

    public static JsVal WrapHandle(string className, int ptr)
    {
        var handle = new JsVal(JsValKind.Handle)
        {
            HandleClass = className,
            HandlePtr = ptr,
            Object = new Dictionary<string, JsVal>(StringComparer.Ordinal),
            ObjectKeys = [],
        };
        handle.Set("__wbg_ptr", FromNumber(ptr));
        return handle;
    }

    public static JsVal From(object? value)
    {
        switch (value)
        {
            case null:
                return Null;
            case JsVal js:
                return js;
            case bool b:
                return FromBoolean(b);
            case string s:
                return FromString(s);
            case byte[] bytes:
                return FromUint8Array(bytes);
            case Func<JsVal[], JsVal> fn:
                return FromFunction(fn);
            case IDictionary dict:
                return FromDictionary(dict);
        }

        var type = value.GetType();
        if (value is IFormattable && IsNumeric(type))
        {
            return FromNumber(Convert.ToDouble(value, CultureInfo.InvariantCulture));
        }

        if (value is IEnumerable enumerable && value is not string)
        {
            var arr = NewArray();
            foreach (var item in enumerable)
            {
                arr.Push(From(item));
            }

            return arr;
        }

        return FromClrObject(value);
    }

    public bool IsUndefined => Kind == JsValKind.Undefined;
    public bool IsNull => Kind == JsValKind.Null;
    public bool IsBoolean => Kind == JsValKind.Boolean;
    public bool IsNumber => Kind == JsValKind.Number;
    public bool IsStringKind => Kind == JsValKind.String;
    public bool IsArrayKind => Kind == JsValKind.Array;
    public bool IsObjectKind => Kind is JsValKind.Object or JsValKind.Handle or JsValKind.ImageData or JsValKind.DomRect or JsValKind.Error;
    public bool IsFunctionKind => Kind == JsValKind.Function;
    public bool IsUint8ArrayKind => Kind == JsValKind.Uint8Array;

    public int Length => Kind switch
    {
        JsValKind.Array => Array!.Count,
        JsValKind.String => String!.Length,
        JsValKind.Uint8Array => Bytes!.Length,
        _ => 0,
    };

    public JsVal Get(int index)
    {
        if (Kind != JsValKind.Array || index < 0 || index >= Array!.Count)
        {
            return Undefined;
        }

        return Array[index];
    }

    public void Set(int index, JsVal value)
    {
        if (Kind != JsValKind.Array)
        {
            return;
        }

        while (Array!.Count <= index)
        {
            Array.Add(Undefined);
        }

        Array[index] = value;
    }

    public int Push(JsVal value)
    {
        if (Kind != JsValKind.Array)
        {
            return 0;
        }

        Array!.Add(value);
        return Array.Count;
    }

    public JsVal Get(string key)
    {
        if (Object is not null && Object.TryGetValue(key, out var value))
        {
            return value;
        }

        return Undefined;
    }

    public void Set(string key, JsVal value)
    {
        if (Object is null || ObjectKeys is null)
        {
            return;
        }

        if (!Object.ContainsKey(key))
        {
            ObjectKeys.Add(key);
        }

        Object[key] = value;
    }

    public IReadOnlyList<string> Keys() => ObjectKeys ?? (IReadOnlyList<string>)System.Array.Empty<string>();

    public JsVal Call(params JsVal[] args) => Function?.Invoke(args) ?? Undefined;

    public override string ToString() => Kind switch
    {
        JsValKind.Undefined => "undefined",
        JsValKind.Null => "null",
        JsValKind.Boolean => Bool ? "true" : "false",
        JsValKind.Number => Number.ToString(CultureInfo.InvariantCulture),
        JsValKind.String => String ?? "",
        JsValKind.Array => "[" + string.Join(", ", Array!.Select(v => v.ToString())) + "]",
        JsValKind.Function => "Function",
        JsValKind.Error => ErrorMessage ?? "Error",
        _ => Kind.ToString(),
    };

    internal string DebugString()
    {
        return Kind switch
        {
            JsValKind.Number or JsValKind.Boolean or JsValKind.Null or JsValKind.Undefined => ToString(),
            JsValKind.String => "\"" + String + "\"",
            JsValKind.Array => ToString(),
            JsValKind.Function => "Function",
            JsValKind.Error => $"{ErrorMessage}\n{ErrorStack}",
            JsValKind.Object => EncodeObject(),
            _ => Kind.ToString(),
        };
    }

    private string EncodeObject()
    {
        try
        {
            var sb = new StringBuilder("Object({");
            var first = true;
            foreach (var key in Keys())
            {
                if (!first)
                {
                    sb.Append(", ");
                }

                first = false;
                sb.Append(key).Append(':').Append(Get(key).DebugString());
            }

            sb.Append("})");
            return sb.ToString();
        }
        catch
        {
            return "Object";
        }
    }

    internal static JsVal Coerce(object? value)
    {
        return value switch
        {
            null => Undefined,
            JsVal js => js,
            bool b => FromBoolean(b),
            string s => FromString(s),
            double d => FromNumber(d),
            float f => FromNumber(f),
            int i => FromNumber(i),
            uint u => FromNumber(u),
            long l => FromNumber(l),
            byte[] bytes => FromUint8Array(bytes),
            _ => From(value),
        };
    }

    private static JsVal FromDictionary(IDictionary dict)
    {
        var obj = NewObject();
        foreach (DictionaryEntry entry in dict)
        {
            obj.Set(Convert.ToString(entry.Key, CultureInfo.InvariantCulture) ?? "", From(entry.Value));
        }

        return obj;
    }

    private static JsVal FromClrObject(object value)
    {
        var obj = NewObject();
        foreach (var prop in value.GetType().GetProperties(BindingFlags.Instance | BindingFlags.Public))
        {
            if (prop.GetIndexParameters().Length != 0)
            {
                continue;
            }

            obj.Set(prop.Name, From(prop.GetValue(value)));
        }

        return obj;
    }

    private static bool IsNumeric(Type type)
    {
        type = Nullable.GetUnderlyingType(type) ?? type;
        return type.IsPrimitive && type != typeof(bool) && type != typeof(char);
    }
}

internal enum JsValKind
{
    Undefined,
    Null,
    Boolean,
    Number,
    String,
    Array,
    Object,
    Uint8Array,
    Function,
    Handle,
    Error,
    ImageData,
    DomRect,
}

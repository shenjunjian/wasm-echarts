using System.Text;
using System.Text.RegularExpressions;
using Wasmtime;

internal static class Program
{
    private static int Main(string[] args)
    {
        var demoRoot = FindDemoRoot();
        var crateRoot = Path.GetFullPath(Path.Combine(demoRoot, "..", "crates", "wasm-echarts"));
        var jsPath = Path.Combine(crateRoot, "pkg", "wasm_echarts.js");
        var wasmPath = Path.Combine(crateRoot, "pkg", "wasm_echarts_bg.wasm");
        if (!File.Exists(jsPath) || !File.Exists(wasmPath))
        {
            Console.Error.WriteLine($"missing pkg glue: {jsPath}");
            return 1;
        }

        var outPath = args.Length > 0
            ? Path.GetFullPath(args[0])
            : Path.Combine(demoRoot, "src", "WasmCharts.Runtime", "WbgImports.g.cs");

        var functions = ParseImports(File.ReadAllText(jsPath));
        var unknown = new List<string>();
        var kinds = new List<(string Name, string Kind)>();
        foreach (var (name, body) in functions)
        {
            var kind = Classify(body);
            if (kind == "NotImplemented")
            {
                unknown.Add(name);
            }

            kinds.Add((name, kind));
        }

        using var engine = new Engine(new Config().WithReferenceTypes(true).WithSIMD(true).WithBulkMemory(true).WithMultiValue(true));
        using var module = Module.FromFile(engine, wasmPath);
        var wasmNames = module.Imports.OfType<FunctionImport>().Select(i => i.Name).ToHashSet();
        foreach (var name in wasmNames)
        {
            if (kinds.All(k => k.Name != name))
            {
                unknown.Add(name + " (wasm-only)");
                kinds.Add((name, "NotImplemented"));
            }
        }

        Directory.CreateDirectory(Path.GetDirectoryName(outPath)!);
        File.WriteAllText(outPath, Render(kinds), new UTF8Encoding(false));
        Console.WriteLine($"wrote {outPath} ({kinds.Count} imports)");
        if (unknown.Count > 0)
        {
            Console.WriteLine("unclassified:");
            foreach (var name in unknown)
            {
                Console.WriteLine("  " + name);
            }
        }

        return 0;
    }

    private static string FindDemoRoot()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir is not null)
        {
            if (File.Exists(Path.Combine(dir.FullName, "NetCoreDemo.slnx"))
                || File.Exists(Path.Combine(dir.FullName, "NetCoreDemo.sln")))
            {
                return dir.FullName;
            }

            dir = dir.Parent;
        }

        return Path.GetFullPath(Path.Combine(AppContext.BaseDirectory, "..", "..", "..", ".."));
    }

    private static List<(string Name, string Body)> ParseImports(string js)
    {
        var start = js.IndexOf("function __wbg_get_imports()", StringComparison.Ordinal);
        if (start < 0)
        {
            throw new InvalidOperationException("glue JS has no __wbg_get_imports");
        }

        var end = js.IndexOf("\n    return {", start, StringComparison.Ordinal);
        var block = js[start..end];
        var matches = Regex.Matches(block, @"^\s{8}(__w(?:bg|bindgen)[A-Za-z0-9_]+):\s*", RegexOptions.Multiline);
        var list = new List<(string, string)>();
        for (var i = 0; i < matches.Count; i++)
        {
            var name = matches[i].Groups[1].Value;
            var bodyStart = matches[i].Index + matches[i].Length;
            var bodyEnd = i + 1 < matches.Count ? matches[i + 1].Index : block.Length;
            list.Add((name, block[bodyStart..bodyEnd]));
        }

        return list;
    }

    private static string Classify(string body)
    {
        if (body.Contains("typeof(v) === 'boolean'", StringComparison.Ordinal)) return "BooleanGet";
        if (body.Contains("new Uint8Array(arg2.buffer", StringComparison.Ordinal)) return "CopyToTypedArray";
        if (body.Contains("debugString", StringComparison.Ordinal)) return "DebugString";
        if (body.Contains("typeof(arg0) === 'function'", StringComparison.Ordinal)) return "IsFunction";
        if (body.Contains("arg0 === null", StringComparison.Ordinal)) return "IsNull";
        if (body.Contains("typeof(val) === 'object' && val !== null", StringComparison.Ordinal)) return "IsObject";
        if (body.Contains("typeof(arg0) === 'string'", StringComparison.Ordinal)) return "IsString";
        if (body.Contains("arg0 === undefined", StringComparison.Ordinal)) return "IsUndefined";
        if (body.Contains("arg0 === arg1", StringComparison.Ordinal)) return "JsvalEq";
        if (body.Contains("typeof(obj) === 'number'", StringComparison.Ordinal)) return "NumberGet";
        if (body.Contains("typeof(obj) === 'string'", StringComparison.Ordinal)) return "StringGet";
        if (body.Contains("throw new Error(getStringFromWasm0", StringComparison.Ordinal)) return "Throw";
        if (body.Contains("_wbg_cb_unref", StringComparison.Ordinal)) return "ClosureUnref";
        if (body.Contains("addEventListener", StringComparison.Ordinal)) return "AddEventListener";
        if (body.Contains("arg0.call(arg1, arg2, arg3)", StringComparison.Ordinal)) return "Call3";
        if (body.Contains("arg0.call(arg1, arg2)", StringComparison.Ordinal)) return "Call2";
        if (body.Contains("arg0.call(arg1)", StringComparison.Ordinal)) return "Call1";
        if (body.Contains("arg0.clientX", StringComparison.Ordinal)) return "ClientX";
        if (body.Contains("arg0.clientY", StringComparison.Ordinal)) return "ClientY";
        if (body.Contains("createElement", StringComparison.Ordinal)) return "CreateElement";
        if (body.Contains("CustomSeriesApi.__wrap", StringComparison.Ordinal)) return "WrapCustomSeriesApi";
        if (body.Contains("arg1.data", StringComparison.Ordinal)) return "ImageDataBytes";
        if (body.Contains("arg0.document", StringComparison.Ordinal)) return "Document";
        if (body.Contains("drawImage", StringComparison.Ordinal)) return "DrawImage";
        if (body.Contains("Element2.__wrap", StringComparison.Ordinal)) return "WrapElement";
        if (body.Contains("console.error(arg0, arg1)", StringComparison.Ordinal)) return "ConsoleError2";
        if (body.Contains("console.error(getStringFromWasm0", StringComparison.Ordinal)) return "ConsoleErrorString";
        if (body.Contains("Array.from", StringComparison.Ordinal)) return "ArrayFrom";
        if (body.Contains("getBoundingClientRect", StringComparison.Ordinal)) return "GetBoundingClientRect";
        if (body.Contains("getContext", StringComparison.Ordinal)) return "GetContext";
        if (body.Contains("getImageData", StringComparison.Ordinal)) return "GetImageData";
        if (body.Contains("Reflect.get", StringComparison.Ordinal)) return "ReflectGet";
        if (body.Contains("instanceof Array", StringComparison.Ordinal)) return "InstanceofArray";
        if (body.Contains("instanceof CanvasRenderingContext2D", StringComparison.Ordinal)) return "InstanceofCanvas2d";
        if (body.Contains("instanceof Element", StringComparison.Ordinal)) return "InstanceofElement";
        if (body.Contains("instanceof Function", StringComparison.Ordinal)) return "InstanceofFunction";
        if (body.Contains("instanceof HTMLCanvasElement", StringComparison.Ordinal)) return "InstanceofCanvas";
        if (body.Contains("instanceof HTMLImageElement", StringComparison.Ordinal)) return "InstanceofImage";
        if (body.Contains("instanceof Object", StringComparison.Ordinal)) return "InstanceofObject";
        if (body.Contains("instanceof Uint8Array", StringComparison.Ordinal)) return "InstanceofUint8Array";
        if (body.Contains("instanceof Window", StringComparison.Ordinal)) return "InstanceofWindow";
        if (body.Contains("Array.isArray", StringComparison.Ordinal)) return "IsArray";
        if (body.Contains("Object.keys", StringComparison.Ordinal)) return "ObjectKeys";
        if (body.Contains("arg0.left", StringComparison.Ordinal)) return "DomLeft";
        if (body.Contains("arg0.length", StringComparison.Ordinal)) return "Length";
        if (body.Contains("naturalHeight", StringComparison.Ordinal)) return "NaturalHeight";
        if (body.Contains("naturalWidth", StringComparison.Ordinal)) return "NaturalWidth";
        if (body.Contains("new Error()", StringComparison.Ordinal)) return "NewError";
        if (body.Contains("new Object()", StringComparison.Ordinal)) return "NewObject";
        if (body.Contains("new Array()", StringComparison.Ordinal)) return "NewArray";
        if (body.Contains("new Uint8Array(getArrayU8FromWasm0", StringComparison.Ordinal)) return "NewUint8ArrayFromSlice";
        if (body.Contains("new Array(arg0", StringComparison.Ordinal)) return "NewArrayWithLength";
        if (body.Contains("new ImageData", StringComparison.Ordinal)) return "NewImageData";
        if (body.Contains("Date.now", StringComparison.Ordinal)) return "DateNow";
        if (body.Contains("Reflect.ownKeys", StringComparison.Ordinal)) return "OwnKeys";
        if (body.Contains("pointerId", StringComparison.Ordinal)) return "PointerId";
        if (body.Contains("Uint8Array.prototype.set.call", StringComparison.Ordinal)) return "TypedArraySetFromWasm";
        if (body.Contains("arg0.push", StringComparison.Ordinal)) return "ArrayPush";
        if (body.Contains("putImageData", StringComparison.Ordinal)) return "PutImageData";
        if (body.Contains("querySelector", StringComparison.Ordinal)) return "QuerySelector";
        if (body.Contains("releasePointerCapture", StringComparison.Ordinal)) return "ReleasePointerCapture";
        if (body.Contains("removeEventListener", StringComparison.Ordinal)) return "RemoveEventListener";
        if (body.Contains("setPointerCapture", StringComparison.Ordinal)) return "SetPointerCapture";
        if (body.Contains("setProperty", StringComparison.Ordinal)) return "SetProperty";
        if (body.Contains("Reflect.set", StringComparison.Ordinal)) return "ReflectSet";
        if (body.Contains("arg0[arg1 >>> 0] = arg2", StringComparison.Ordinal)) return "ArraySet";
        if (body.Contains("arg0.height = arg1", StringComparison.Ordinal)) return "SetHeight";
        if (body.Contains("arg0.width = arg1", StringComparison.Ordinal)) return "SetWidth";
        if (body.Contains("arg1.stack", StringComparison.Ordinal)) return "ErrorStack";
        if (body.Contains("typeof globalThis", StringComparison.Ordinal)) return "GlobalThis";
        if (body.Contains("typeof global ===", StringComparison.Ordinal)) return "Global";
        if (body.Contains("typeof self ===", StringComparison.Ordinal)) return "Self";
        if (body.Contains("typeof window ===", StringComparison.Ordinal)) return "Window";
        if (body.Contains("arg0.style", StringComparison.Ordinal)) return "Style";
        if (body.Contains("arg0.top", StringComparison.Ordinal)) return "DomTop";
        if (body.Contains("console.warn", StringComparison.Ordinal)) return "ConsoleWarn";
        if (body.Contains("table.grow(4)", StringComparison.Ordinal)) return "InitExternrefTable";
        if (body.Contains("F64 -> Externref", StringComparison.Ordinal)) return "CastF64";
        if (body.Contains("Ref(String) -> Externref", StringComparison.Ordinal)) return "CastString";
        if (body.Contains("makeMutClosure", StringComparison.Ordinal)) return "CastClosure";
        if (body.Contains("arg0[arg1 >>> 0]", StringComparison.Ordinal)) return "ArrayGet";
        if (body.Contains("arg0.width", StringComparison.Ordinal) && body.Contains("_assertNum", StringComparison.Ordinal))
        {
            return "WidthI32";
        }

        if (body.Contains("arg0.height", StringComparison.Ordinal) && body.Contains("_assertNum", StringComparison.Ordinal))
        {
            return "HeightI32";
        }

        if (body.Contains("arg0.width", StringComparison.Ordinal)) return "WidthF64";
        if (body.Contains("arg0.height", StringComparison.Ordinal)) return "HeightF64";
        return "NotImplemented";
    }

    private static string Render(List<(string Name, string Kind)> kinds)
    {
        var sb = new StringBuilder();
        sb.AppendLine("// <auto-generated />");
        sb.AppendLine("// Generated from crates/wasm-echarts/pkg/wasm_echarts.js — re-run tools/GenWbgImports after wasm-pack.");
        sb.AppendLine("#nullable enable");
        sb.AppendLine("using Wasmtime;");
        sb.AppendLine();
        sb.AppendLine("namespace WasmCharts.Runtime;");
        sb.AppendLine();
        sb.AppendLine("internal static partial class WbgImports");
        sb.AppendLine("{");
        sb.AppendLine("    private static readonly Dictionary<string, WbgKind> Kinds = new()");
        sb.AppendLine("    {");
        foreach (var (name, kind) in kinds.OrderBy(k => k.Name, StringComparer.Ordinal))
        {
            sb.AppendLine($"        [\"{name}\"] = WbgKind.{kind},");
        }

        sb.AppendLine("    };");
        sb.AppendLine();
        sb.AppendLine("    public static void Define(Linker linker, Module module, WbgHost host)");
        sb.AppendLine("    {");
        sb.AppendLine("        foreach (var import in module.Imports)");
        sb.AppendLine("        {");
        sb.AppendLine("            if (import is not FunctionImport fn)");
        sb.AppendLine("            {");
        sb.AppendLine("                continue;");
        sb.AppendLine("            }");
        sb.AppendLine();
        sb.AppendLine("            var name = import.Name;");
        sb.AppendLine("            var kind = Kinds.GetValueOrDefault(name, WbgKind.NotImplemented);");
        sb.AppendLine("            linker.DefineFunction(");
        sb.AppendLine("                import.ModuleName,");
        sb.AppendLine("                name,");
        sb.AppendLine("                (Caller caller, ReadOnlySpan<ValueBox> args, Span<ValueBox> results) =>");
        sb.AppendLine("                    host.Invoke(kind, name, caller, args, results),");
        sb.AppendLine("                fn.Parameters,");
        sb.AppendLine("                fn.Results);");
        sb.AppendLine("        }");
        sb.AppendLine("    }");
        sb.AppendLine("}");
        return sb.ToString();
    }
}

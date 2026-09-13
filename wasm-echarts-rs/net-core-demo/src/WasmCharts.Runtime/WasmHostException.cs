namespace WasmCharts.Runtime;

public sealed class WasmHostException : Exception
{
    public WasmHostException(string message) : base(message)
    {
    }

    public WasmHostException(string message, Exception inner) : base(message, inner)
    {
    }
}

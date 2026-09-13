namespace NetCoreDemo.Wpf.Catalog;

public enum DemoProduct
{
    Zrender,
    Echarts,
}

public enum DemoStatus
{
    Ready,
    OptionOnly,
    NeedsCallback,
    UnsupportedHost,
    Pending,
}

public sealed class DemoEntry
{
    public required string Id { get; init; }
    public required string Title { get; init; }
    public string? Description { get; init; }
    public required DemoProduct Product { get; init; }
    public string? GroupId { get; init; }
    public string? GroupTitle { get; init; }
    public DemoStatus Status { get; init; }
}

public sealed class DemoGroup
{
    public required string Id { get; init; }
    public required string Title { get; init; }
    public required IReadOnlyList<DemoEntry> Examples { get; init; }
}

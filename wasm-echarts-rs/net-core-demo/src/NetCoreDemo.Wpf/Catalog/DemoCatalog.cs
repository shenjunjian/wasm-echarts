namespace NetCoreDemo.Wpf.Catalog;

internal static class DemoCatalog
{
    public static IReadOnlyList<DemoEntry> Zrender { get; } =
    [
        Item("hello_world", "Hello World!", "Circle + Rect + RadialGradient", DemoStatus.Ready),
        Item("animation", "Animation", "animate().when().start() 写入最后一组 when（圆停在右侧)"),
        Item("bounding_box", "Bounding Box", "init(canvas) + draggable Circle，实时 Group boundingRect"),
        Item("clip_path", "ClipPath", "Circle.setClipPath(Heart)"),
        Item("glitched_text", "Glitched Text", "Text + attr(position/shape/style)"),
        Item("particles", "Particles", "animate/during/done 终态：during(t=1) 后 done 立刻 remove"),
        Item("shapes", "基础图形 shapes", "Rect.r / Sector.r0、原型链 Rect instanceof Path、Group.x"),
        Item("text", "文本 text", "registerFont + new Text({ style })"),
        Item("sector", "扇区 sector", "循环 new Sector 饼图扇区"),
        Item("hit", "命中检测 hit", "findHover + 鼠标移动"),
        Item("state", "状态 state", "setStateStyle + useState emphasis"),
    ];

    private static IReadOnlyList<DemoGroup>? _echartsGroups;

    public static IReadOnlyList<DemoGroup> EchartsGroups => _echartsGroups ??= BuildEchartsGroups();

    public static IReadOnlyList<DemoEntry> Echarts =>
        EchartsGroups.SelectMany(group => group.Examples).ToArray();

    public static DemoEntry DefaultZrender => Zrender[0];

    public static DemoEntry DefaultEcharts =>
        Echarts.First(item => item.Id == "line");

    public static string StatusLabel(DemoStatus status) => status switch
    {
        DemoStatus.Ready => "Ready",
        DemoStatus.OptionOnly => "OptionOnly",
        DemoStatus.NeedsCallback => "NeedsCallback",
        DemoStatus.UnsupportedHost => "UnsupportedHost",
        _ => "Pending",
    };

    private static DemoEntry Item(string id, string title, string description, DemoStatus status = DemoStatus.Pending)
    {
        return new DemoEntry
        {
            Id = id,
            Title = title,
            Description = description,
            Product = DemoProduct.Zrender,
            Status = status,
        };
    }

    private static IReadOnlyList<DemoGroup> BuildEchartsGroups()
    {
        var seen = new HashSet<string>(StringComparer.Ordinal);
        var groups = new List<DemoGroup>
        {
            new()
            {
                Id = "cat-interaction",
                Title = "综合功能",
                Examples = Interaction().Select(item => Track(seen, item)).ToArray(),
            },
        };

        foreach (var spec in CategorySpecs)
        {
            var examples = new List<DemoEntry>();
            foreach (var handmade in spec.Handmade)
            {
                if (!seen.Add(handmade.Id))
                {
                    continue;
                }

                examples.Add(new DemoEntry
                {
                    Id = handmade.Id,
                    Title = handmade.Title,
                    Description = handmade.Description,
                    Product = DemoProduct.Echarts,
                    GroupId = $"cat-{spec.Category}",
                    GroupTitle = spec.Title,
                    Status = handmade.Status,
                });
            }

            foreach (var official in EchartsOfficialCatalog.Items.Where(item => item.Category == spec.Category))
            {
                if (!seen.Add(official.Id))
                {
                    continue;
                }

                examples.Add(new DemoEntry
                {
                    Id = official.Id,
                    Title = official.Title,
                    Description = official.Description,
                    Product = DemoProduct.Echarts,
                    GroupId = $"cat-{spec.Category}",
                    GroupTitle = spec.Title,
                    Status = StatusForOfficial(official.Id),
                });
            }

            if (examples.Count == 0)
            {
                continue;
            }

            groups.Add(new DemoGroup
            {
                Id = $"cat-{spec.Category}",
                Title = spec.Title,
                Examples = examples,
            });
        }

        return groups;
    }

    private static DemoEntry Track(HashSet<string> seen, DemoEntry item)
    {
        seen.Add(item.Id);
        return item;
    }

    private static DemoStatus StatusForOfficial(string id)
    {
        if (id.Contains("bmap", StringComparison.OrdinalIgnoreCase))
        {
            return DemoStatus.UnsupportedHost;
        }

        return DemoStatus.Pending;
    }

    private static IEnumerable<DemoEntry> Interaction()
    {
        yield return EchartsItem("cat-interaction", "综合功能", "fonts", "多字体",
            "registerFont 引入雅黑 / 宋体 / 楷体，分别用于标题、副标题、图例与轴单位");
        yield return EchartsItem("cat-interaction", "综合功能", "interactive", "点击 / tooltip / zoom",
            "use() 提示 + on(click) / toggleSelect / tooltip / wheel zoom");
        yield return EchartsItem("cat-interaction", "综合功能", "merge", "setOption 合并",
            "深合并、notMerge: true、dispose 后再 init");
        yield return EchartsItem("cat-interaction", "综合功能", "bench", "性能基准",
            "benchmarkRender 30 次均值");
    }

    private static DemoEntry EchartsItem(
        string groupId,
        string groupTitle,
        string id,
        string title,
        string description,
        DemoStatus status = DemoStatus.Pending)
    {
        return new DemoEntry
        {
            Id = id,
            Title = title,
            Description = description,
            Product = DemoProduct.Echarts,
            GroupId = groupId,
            GroupTitle = groupTitle,
            Status = status,
        };
    }

    private static readonly CategorySpec[] CategorySpecs =
    [
        new("line", "折线图", Ready("line", "基础折线", "category 轴折线图")),
        new("bar", "柱状图", Ready("bar", "基础柱状", "value 轴柱状图")),
        new("pie", "饼图", Ready("pie", "基础饼图", "扇区饼图")),
        new("scatter", "散点图", Ready("scatter", "基础散点", "value 轴散点图")),
        new("candlestick", "K 线图"),
        new("boxplot", "盒须图"),
        new("heatmap", "热力图"),
        new("pictorialBar", "象形柱图"),
        new("gauge", "仪表盘"),
        new("radar", "雷达图"),
        new("funnel", "漏斗图"),
        new("chord", "和弦图"),
        new("sunburst", "旭日图"),
        new("tree", "树图"),
        new("treemap", "矩形树图"),
        new("graph", "关系图"),
        new("sankey", "桑基图"),
        new("themeRiver", "主题河流"),
        new("calendar", "日历"),
        new("matrix", "矩阵"),
        new("parallel", "平行坐标"),
        new("map", "地图"),
        new("geo", "地理坐标"),
        new("lines", "路径图"),
        new("custom", "自定义系列"),
        new("dataset", "数据集"),
        new("graphic", "图形组件"),
    ];

    private static DemoEntry Ready(string id, string title, string description)
    {
        return new DemoEntry
        {
            Id = id,
            Title = title,
            Description = description,
            Product = DemoProduct.Echarts,
            Status = DemoStatus.Ready,
        };
    }

    private readonly record struct CategorySpec(string Category, string Title, DemoEntry[] Handmade)
    {
        public CategorySpec(string category, string title)
            : this(category, title, [])
        {
        }

        public CategorySpec(string category, string title, DemoEntry handmade)
            : this(category, title, [handmade])
        {
        }
    }
}

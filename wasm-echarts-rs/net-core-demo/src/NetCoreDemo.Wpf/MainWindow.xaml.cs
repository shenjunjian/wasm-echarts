using System.Windows;
using System.Windows.Controls;
using System.Windows.Media;
using ICSharpCode.AvalonEdit.Highlighting;
using NetCoreDemo.Wpf.Catalog;
using NetCoreDemo.Wpf.Demos;
using NetCoreDemo.Wpf.Rendering;
using WasmCharts.Runtime;

namespace NetCoreDemo.Wpf;

public partial class MainWindow : Window
{
    private static readonly SolidColorBrush Accent = Brush("#2563EB");
    private static readonly SolidColorBrush AccentSoft = Brush("#EFF6FF");
    private static readonly SolidColorBrush TextBrush = Brush("#1A1D24");
    private static readonly SolidColorBrush Muted = Brush("#5C6370");
    private static readonly SolidColorBrush BorderBrushColor = Brush("#E2E5EA");
    private static readonly SolidColorBrush Surface = Brush("#FFFFFF");

    private readonly object _wasmLock = new();
    private readonly HashSet<string> _expandedGroups = [];
    private readonly List<IDisposable> _sessionDisposables = [];

    private DemoProduct _product = DemoProduct.Zrender;
    private DemoEntry? _active;
    private int _renderSeq;
    private WasmEchartsModule? _echarts;
    private WasmZrenderModule? _zrender;

    public MainWindow()
    {
        InitializeComponent();
        SourceEditor.SyntaxHighlighting = HighlightingManager.Instance.GetDefinition("C#");
        SourceEditor.Options.EnableHyperlinks = false;
        SourceEditor.Options.EnableEmailHyperlinks = false;
        SourceEditor.LineNumbersForeground = Brush("#64748B");
        foreach (var group in DemoCatalog.EchartsGroups)
        {
            _expandedGroups.Add(group.Id);
        }

        Loaded += (_, _) => SelectProduct(DemoProduct.Zrender);
        Closed += (_, _) => DisposeWasm();
    }

    private void OnZrenderClick(object sender, RoutedEventArgs e) => SelectProduct(DemoProduct.Zrender);

    private void OnEchartsClick(object sender, RoutedEventArgs e) => SelectProduct(DemoProduct.Echarts);

    private async void OnCopyClick(object sender, RoutedEventArgs e)
    {
        try
        {
            Clipboard.SetText(SourceEditor.Text ?? "");
            CopyButton.Content = "已复制";
            await Task.Delay(1500);
            CopyButton.Content = "复制";
        }
        catch
        {
            CopyButton.Content = "复制失败";
        }
    }

    private void SelectProduct(DemoProduct product)
    {
        _product = product;
        ZrenderButton.Tag = product == DemoProduct.Zrender ? "active" : null;
        EchartsButton.Tag = product == DemoProduct.Echarts ? "active" : null;
        StyleProductButton(ZrenderButton, product == DemoProduct.Zrender);
        StyleProductButton(EchartsButton, product == DemoProduct.Echarts);

        if (product == DemoProduct.Zrender)
        {
            SidebarTitle.Text = "wasm-zrender 实例";
            SidebarDesc.Text = "对齐官方 zrender 示例；动画为终态语义。右侧查看 C# 接入代码与 RGBA 预览。";
            SelectDemo(DemoCatalog.DefaultZrender);
        }
        else
        {
            SidebarTitle.Text = "wasm-echarts 实例";
            SidebarDesc.Text = "分组与站点 OFFICIAL_CATEGORY_GROUPS 一致。未移植项保留菜单并显示状态。";
            SelectDemo(DemoCatalog.DefaultEcharts);
        }
    }

    private void RebuildNav()
    {
        NavPanel.Children.Clear();
        if (_product == DemoProduct.Zrender)
        {
            foreach (var item in DemoCatalog.Zrender)
            {
                NavPanel.Children.Add(CreateLeaf(item));
            }

            return;
        }

        foreach (var group in DemoCatalog.EchartsGroups)
        {
            NavPanel.Children.Add(CreateGroup(group));
        }
    }

    private UIElement CreateGroup(DemoGroup group)
    {
        var expanded = _expandedGroups.Contains(group.Id);
        var box = new StackPanel { Margin = new Thickness(0, 0, 0, 6) };
        var toggle = new Button
        {
            Tag = group.Id,
            Padding = new Thickness(8, 6, 8, 6),
            Background = Brushes.Transparent,
            BorderThickness = new Thickness(0),
            HorizontalContentAlignment = HorizontalAlignment.Stretch,
            Cursor = System.Windows.Input.Cursors.Hand,
            Content = new Grid
            {
                ColumnDefinitions =
                {
                    new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) },
                    new ColumnDefinition { Width = GridLength.Auto },
                },
                Children =
                {
                    Named(new TextBlock
                    {
                        Text = group.Title,
                        FontWeight = FontWeights.SemiBold,
                        FontSize = 13,
                        Foreground = TextBrush,
                    }, 0),
                    Named(new TextBlock
                    {
                        Text = group.Examples.Count.ToString(),
                        FontSize = 11,
                        Foreground = Muted,
                        Margin = new Thickness(8, 0, 0, 0),
                    }, 1),
                },
            },
        };
        toggle.Click += (_, _) =>
        {
            if (!_expandedGroups.Add(group.Id))
            {
                _expandedGroups.Remove(group.Id);
            }

            RebuildNav();
        };
        box.Children.Add(toggle);
        if (expanded)
        {
            var children = new StackPanel { Margin = new Thickness(8, 0, 0, 0) };
            foreach (var item in group.Examples)
            {
                children.Children.Add(CreateLeaf(item));
            }

            box.Children.Add(children);
        }

        return box;
    }

    private Button CreateLeaf(DemoEntry item)
    {
        var active = _active?.Id == item.Id;
        var title = new TextBlock
        {
            Text = item.Title,
            FontWeight = FontWeights.SemiBold,
            FontSize = 13,
            Foreground = TextBrush,
        };
        var desc = new TextBlock
        {
            Text = item.Description ?? "",
            FontSize = 11,
            Foreground = Muted,
            TextWrapping = TextWrapping.Wrap,
            Margin = new Thickness(0, 2, 0, 0),
        };
        var status = new TextBlock
        {
            Text = DemoCatalog.StatusLabel(item.Status),
            FontSize = 10,
            Foreground = item.Status == DemoStatus.Ready ? Accent : Muted,
            Margin = new Thickness(0, 4, 0, 0),
        };
        var button = new Button
        {
            Tag = item,
            Padding = new Thickness(10, 8, 10, 8),
            Margin = new Thickness(0, 0, 0, 4),
            HorizontalContentAlignment = HorizontalAlignment.Stretch,
            BorderBrush = active ? Brush("#BFDBFE") : Brushes.Transparent,
            BorderThickness = new Thickness(1),
            Background = active ? AccentSoft : Brushes.Transparent,
            Cursor = System.Windows.Input.Cursors.Hand,
            Content = new StackPanel { Children = { title, desc, status } },
        };
        button.Click += (_, _) => SelectDemo(item);
        return button;
    }

    private void SelectDemo(DemoEntry entry)
    {
        _active = entry;
        RebuildNav();
        var demo = DemoRegistry.Resolve(entry);
        SourceEditor.Text = demo.Source;
        _ = RenderAsync(entry, demo);
    }

    private async Task RenderAsync(DemoEntry entry, IDemo demo)
    {
        var seq = ++_renderSeq;
        PreviewImage.Source = null;
        ErrorHost.Visibility = Visibility.Collapsed;
        StatusText.Text = "渲染中…";

        var width = entry.Product == DemoProduct.Zrender ? 640 : 480;
        var height = entry.Product == DemoProduct.Zrender ? 400 : 360;

        RenderResult result;
        try
        {
            if (demo is PendingDemo)
            {
                result = demo.Render(new RenderContext
                {
                    Echarts = null!,
                    Zrender = null!,
                    Width = width,
                    Height = height,
                    Track = static _ => { },
                });
            }
            else
            {
                result = await Task.Run(() => RenderOnWorker(entry, demo, width, height));
            }
        }
        catch (Exception ex)
        {
            result = new RenderResult(null, width, height, ex.Message);
        }

        if (seq != _renderSeq)
        {
            return;
        }

        if (result.Error is not null)
        {
            ErrorBanner.Text = result.Error;
            ErrorHost.Visibility = Visibility.Visible;
            PreviewImage.Source = null;
            StatusText.Text = DemoCatalog.StatusLabel(entry.Status);
            return;
        }

        if (result.Rgba is null || result.Rgba.Length == 0)
        {
            ErrorBanner.Text = "refresh() 返回空缓冲区";
            ErrorHost.Visibility = Visibility.Visible;
            StatusText.Text = "错误";
            return;
        }

        PreviewImage.Source = RgbaBitmap.FromRgba(result.Rgba, result.Width, result.Height);
        StatusText.Text = $"{result.Width}×{result.Height}  {DemoCatalog.StatusLabel(entry.Status)}";
    }

    private RenderResult RenderOnWorker(DemoEntry entry, IDemo demo, int width, int height)
    {
        lock (_wasmLock)
        {
            ClearSession();
            var context = new RenderContext
            {
                Echarts = EnsureEcharts(),
                Zrender = EnsureZrender(),
                Width = width,
                Height = height,
                Track = item => _sessionDisposables.Add(item),
            };
            return demo.Render(context);
        }
    }

    private WasmEchartsModule EnsureEcharts()
    {
        _echarts ??= WasmEchartsModule.Load();
        return _echarts;
    }

    private WasmZrenderModule EnsureZrender()
    {
        _zrender ??= WasmZrenderModule.Load();
        return _zrender;
    }

    private void ClearSession()
    {
        foreach (var item in _sessionDisposables)
        {
            item.Dispose();
        }

        _sessionDisposables.Clear();
    }

    private void DisposeWasm()
    {
        lock (_wasmLock)
        {
            ClearSession();
            _echarts?.Dispose();
            _zrender?.Dispose();
            _echarts = null;
            _zrender = null;
        }
    }

    private static void StyleProductButton(Button button, bool active)
    {
        button.Background = active ? Accent : Surface;
        button.Foreground = active ? Brushes.White : TextBrush;
        button.BorderBrush = active ? Accent : BorderBrushColor;
    }

    private static TextBlock Named(TextBlock block, int column)
    {
        Grid.SetColumn(block, column);
        return block;
    }

    private static SolidColorBrush Brush(string hex)
    {
        var brush = new SolidColorBrush((Color)ColorConverter.ConvertFromString(hex));
        brush.Freeze();
        return brush;
    }
}
